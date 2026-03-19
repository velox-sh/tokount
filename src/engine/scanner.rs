use std::arch::x86_64::*;

use memchr::memchr;
use memchr::memchr2;
use memchr::memchr3;

/// Find the next byte that could start a token
///
/// Dispatches to memchr/memchr2/memchr3 (guaranteed SIMD) for <=3 needles
/// For larger sets, uses an SSE2 loop on x86-64 (SSE2 is baseline there)
/// or a scalar fallback on other targets
#[inline]
pub fn find_interesting(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    match needles {
        [] => None,
        [a] => memchr(*a, bytes),
        [a, b] => memchr2(*a, *b, bytes),
        [a, b, c] => memchr3(*a, *b, *c, bytes),
        _ => find_interesting_wide(bytes, needles),
    }
}

/// SSE2 implementation for >3 needles
/// SSE2 is part of the x86-64 ABI so no runtime detection needed
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[allow(unsafe_code)] // required for std::arch intrinsics
unsafe fn find_interesting_sse2(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    // function only callable on x86-64 with SSE2 (enforced by #[target_feature])
    // ptr arithmetic stays within the `bytes` slice (loop bound: i + 16 <= len)
    unsafe {
        let len = bytes.len();
        let ptr = bytes.as_ptr();
        let mut i = 0;

        while i + 16 <= len {
            let chunk = _mm_loadu_si128(ptr.add(i).cast::<__m128i>());
            let mut hit = _mm_setzero_si128();

            for &n in needles {
                hit = _mm_or_si128(hit, _mm_cmpeq_epi8(chunk, _mm_set1_epi8(n as i8)));
            }

            let mask = _mm_movemask_epi8(hit) as u32;
            if mask != 0 {
                return Some(i + mask.trailing_zeros() as usize);
            }

            i += 16;
        }

        // scalar tail (< 16 bytes remaining)
        bytes[i..]
            .iter()
            .position(|b| needles.contains(b))
            .map(|p| i + p)
    }
}

#[cfg(target_arch = "x86_64")]
#[inline]
fn find_interesting_wide(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    // SSE2 is part of the x86-64 ABI baseline
    #[allow(unsafe_code)]
    unsafe {
        find_interesting_sse2(bytes, needles)
    }
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
fn find_interesting_wide(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    bytes.iter().position(|b| needles.contains(b))
}

/// Find next newline (LineComment state)
#[inline]
pub fn find_newline(bytes: &[u8]) -> Option<usize> {
    memchr(b'\n', bytes)
}

/// Find next newline or close byte (BlockComment state)
#[inline]
pub fn find_newline_or(bytes: &[u8], close_byte: u8) -> Option<usize> {
    if close_byte == b'\n' {
        memchr(b'\n', bytes)
    } else {
        memchr2(b'\n', close_byte, bytes)
    }
}

/// Find next newline, close byte, or backslash (String state)
#[inline]
pub fn find_string_end(bytes: &[u8], close_byte: u8) -> Option<usize> {
    memchr3(b'\n', close_byte, b'\\', bytes)
}

/// Find next newline or close byte, no backslash check (raw/verbatim strings)
#[inline]
pub fn find_string_end_no_escape(bytes: &[u8], close_byte: u8) -> Option<usize> {
    memchr2(b'\n', close_byte, bytes)
}

/// Find next newline, open-delimiter first-byte, or close-delimiter first-byte
/// (nested block comment state)
#[inline]
pub fn find_nested_block(bytes: &[u8], open_first: u8, close_first: u8) -> Option<usize> {
    if open_first == close_first {
        memchr2(b'\n', open_first, bytes)
    } else {
        memchr3(b'\n', open_first, close_first, bytes)
    }
}
