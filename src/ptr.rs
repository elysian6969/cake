//! Pointer-related functions.

/// Convenience method for performing `&*pointer`.
///
/// # Safety
///
/// Caller must ensure `pointer` is non-null, valid, well-aligned, etc.
#[inline]
pub const unsafe fn reborrow<'a, T>(pointer: *const T) -> &'a T {
    &*pointer
}
