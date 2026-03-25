#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use memchr::memchr;
use memchr::memchr2;
use memchr::memchr3;

// upper bound on interest_bytes length across all languages
const MAX_NEEDLES: usize = 16;

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[expect(unsafe_code)]
unsafe fn find_interesting_avx2(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    unsafe {
        let len = bytes.len();
        let ptr = bytes.as_ptr();
        let n = needles.len().min(MAX_NEEDLES);

        // needle vecs are loop-invariant; hoisting avoids recreating them per 32-byte chunk
        let zero = _mm256_setzero_si256();
        let mut needle_vecs = [zero; MAX_NEEDLES];

        for (i, &needle) in needles.iter().take(n).enumerate() {
            needle_vecs[i] = _mm256_set1_epi8(needle as i8);
        }

        let mut i = 0;
        while i + 32 <= len {
            let chunk = _mm256_loadu_si256(ptr.add(i).cast::<__m256i>());
            let mut hit = zero;

            for nv in needle_vecs.iter().take(n) {
                hit = _mm256_or_si256(hit, _mm256_cmpeq_epi8(chunk, *nv));
            }

            let mask = _mm256_movemask_epi8(hit) as u32;
            if mask != 0 {
                return Some(i + mask.trailing_zeros() as usize);
            }

            i += 32;
        }

        // scalar tail (< 32 bytes remaining)
        bytes[i..]
            .iter()
            .position(|b| needles.contains(b))
            .map(|p| i + p)
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "sse2")]
#[expect(unsafe_code)]
unsafe fn find_interesting_sse2(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    unsafe {
        let len = bytes.len();
        let ptr = bytes.as_ptr();
        let n = needles.len().min(MAX_NEEDLES);

        // needle vecs are loop-invariant; hoisting avoids recreating them per 16-byte chunk
        let zero = _mm_setzero_si128();
        let mut needle_vecs = [zero; MAX_NEEDLES];

        for (i, &needle) in needles.iter().take(n).enumerate() {
            needle_vecs[i] = _mm_set1_epi8(needle as i8);
        }

        let mut i = 0;
        while i + 16 <= len {
            let chunk = _mm_loadu_si128(ptr.add(i).cast::<__m128i>());
            let mut hit = zero;

            for nv in needle_vecs.iter().take(n) {
                hit = _mm_or_si128(hit, _mm_cmpeq_epi8(chunk, *nv));
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
    #[expect(unsafe_code)]
    // feature detected at runtime, so safe to call either SIMD variant
    if is_x86_feature_detected!("avx2") {
        unsafe { find_interesting_avx2(bytes, needles) }
    } else {
        unsafe { find_interesting_sse2(bytes, needles) }
    }
}

#[cfg(not(target_arch = "x86_64"))]
#[inline]
fn find_interesting_wide(bytes: &[u8], needles: &[u8]) -> Option<usize> {
    bytes.iter().position(|b| needles.contains(b))
}

/// Find the next occurrence of any byte in `needles`
/// (memchr/2/3 for ≤3 needles (guaranteed SIMD); AVX2/SSE2 loop for >3 on x86-64; scalar elsewhere)
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

/// Find the next newline
#[inline]
pub fn find_newline(bytes: &[u8]) -> Option<usize> {
    memchr(b'\n', bytes)
}

/// Find the next newline or `close_byte` (used for line comment terminators)
#[inline]
pub fn find_newline_or(bytes: &[u8], close_byte: u8) -> Option<usize> {
    if close_byte == b'\n' {
        memchr(b'\n', bytes)
    } else {
        memchr2(b'\n', close_byte, bytes)
    }
}

/// Find the end of a string literal with escape support (`\`, newline, or close)
#[inline]
pub fn find_string_end(bytes: &[u8], close_byte: u8) -> Option<usize> {
    memchr3(b'\n', close_byte, b'\\', bytes)
}

/// Find the end of a raw string literal without escape support (newline or close)
#[inline]
pub fn find_string_end_no_escape(bytes: &[u8], close_byte: u8) -> Option<usize> {
    memchr2(b'\n', close_byte, bytes)
}

/// Find the next token relevant to nested block comment tracking
#[inline]
pub fn find_nested_block(bytes: &[u8], open_first: u8, close_first: u8) -> Option<usize> {
    if open_first == close_first {
        memchr2(b'\n', open_first, bytes)
    } else {
        memchr3(b'\n', open_first, close_first, bytes)
    }
}
