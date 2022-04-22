//! Raw pointer utilities.

/// Convenience method for performing `&*pointer`.
///
/// # Safety
///
/// Caller must ensure `pointer` is non-null, valid, well-aligned, etc.
#[inline]
pub const unsafe fn reborrow<'a, T>(pointer: *const T) -> &'a T {
    &*pointer
}

/// Convenience method for performing `&mut *pointer`, mutable variant.
///
/// # Safety
///
/// Caller must ensure `pointer` is non-null, valid, well-aligned, etc.
#[inline]
pub const unsafe fn reborrow_mut<'a, T>(pointer: *mut T) -> &'a mut T {
    &mut *pointer
}
