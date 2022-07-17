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
