use memchr::memchr;
use memchr::memchr2;
use memchr::memchr3;

/// Find next byte matching the interest mask
/// (SIMD-accelerated via LLVM auto-vectorization)
#[inline]
#[allow(unsafe_code)] // b is u8, so b as usize is always in 0..256; bounds check is provably dead
pub fn find_interesting(bytes: &[u8], mask: &[bool; 256]) -> Option<usize> {
    bytes.iter().position(|&b| {
        // safety: b as usize is always in 0..256
        // the mask is exactly 256 elements
        unsafe { *mask.get_unchecked(b as usize) }
    })
}

/// Find next newline
/// (LineComment state)
#[inline]
pub fn find_newline(bytes: &[u8]) -> Option<usize> {
    memchr(b'\n', bytes)
}

/// Find next newline or close byte
/// (BlockComment state)
#[inline]
pub fn find_newline_or(bytes: &[u8], close_byte: u8) -> Option<usize> {
    if close_byte == b'\n' {
        memchr(b'\n', bytes)
    } else {
        memchr2(b'\n', close_byte, bytes)
    }
}

/// Find next newline, close byte, or backslash
/// (String state)
#[inline]
pub fn find_string_end(bytes: &[u8], close_byte: u8) -> Option<usize> {
    memchr3(b'\n', close_byte, b'\\', bytes)
}

/// Find next newline or close byte, no backslash check
/// (raw/verbatim strings)
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
