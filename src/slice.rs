//! Helper functions and types for slices.

use crate::tuple;
use core::slice;

/// Convert a reference to a contiguous tuple to a slice, without copying.
#[inline]
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as tuple::Array>::Element]
where
    T: ~const tuple::Array,
{
    let address = (tuple as *const T).cast();
    let len = tuple::len::<T>();

    unsafe { slice::from_raw_parts(address, len) }
}

/// Convert a mutable reference to a contiguous tuple to a slice, without copying.
#[inline]
pub const fn from_tuple_mut<T>(tuple: &mut T) -> &mut [<T as tuple::Array>::Element]
where
    T: ~const tuple::Array,
{
    let address = (tuple as *mut T).cast();
    let len = tuple::len::<T>();

    unsafe { slice::from_raw_parts_mut(address, len) }
}
