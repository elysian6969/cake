use crate::slice;
use core::str;

/// Construct a str from two pointers.
///
/// # Safety
///
/// Both slices must be of the same allocation.
pub const unsafe fn conjoin<'a>(base: *const u8, address: *const u8) -> &'a str {
    str::from_utf8_unchecked(slice::conjoin(base, address))
}

/// `str::as_ptr` but pointing to the end of the string.
pub const fn as_ptr_end(string: &str) -> *const u8 {
    slice::as_ptr_end(string.as_bytes())
}
