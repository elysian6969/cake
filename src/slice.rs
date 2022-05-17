//! Helper functions and types for slices.

use crate::tuple::TupleArray;
use core::slice;

/// Convert a typle of all the same type to a slice, without copying.
#[inline]
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as TupleArray>::Element]
where
    T: ~const TupleArray,
{
    unsafe { slice::from_raw_parts(T::as_ptr(tuple), T::LEN) }
}

/// Construct a slice from two pointers.
///
/// # Safety
///
/// Everything from `ptr::offset_from`.
pub const unsafe fn conjoin<'a, T>(base: *const T, address: *const T) -> &'a [T] {
    slice::from_raw_parts(base, address.offset_from(base) as usize)
}

/// `slice::as_ptr` but pointing to the end of the slice.
pub const fn as_ptr_end<T>(slice: &[T]) -> *const T {
    match slice.last() {
        Some(last) => last as *const T,
        None => slice.as_ptr(),
    }
}

/// Convert a slice to it's pointer and length.
pub const fn to_raw_parts<'a, T>(slice: &'a [T]) -> (*const T, usize)
where
    T: 'a,
{
    (slice.as_ptr(), slice.len())
}
