//! Helper functions and types for slices.

use crate::mem::Layout;
use crate::tuple;
use core::slice::SliceIndex;
use core::{ptr, slice};

/// Convert a reference to a contiguous tuple to a slice, without copying.
#[inline]
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as tuple::Array>::Element]
where
    T: tuple::Array,
{
    let address = (tuple as *const T).cast();
    let len = tuple::len::<T>();

    unsafe { slice::from_raw_parts(address, len) }
}

/// Convert a mutable reference to a contiguous tuple to a slice, without copying.
#[inline]
pub const fn from_tuple_mut<T>(tuple: &mut T) -> &mut [<T as tuple::Array>::Element]
where
    T: tuple::Array,
{
    let address = (tuple as *mut T).cast();
    let len = tuple::len::<T>();

    unsafe { slice::from_raw_parts_mut(address, len) }
}

/// Copies elements from one part of the slice to another part of itself.
///
/// # Safety
///
/// All indicies must be valid.
#[inline]
pub const unsafe fn copy_within_unchecked<T, I>(slice: &mut [T], src: I, dst: usize)
where
    T: Copy,
    I: ~const SliceIndex<[T]>,
{
    let src = slice.get_unchecked(src);
    let src_ptr = src as *const _;
    let src_len = Layout::from_val_raw(src_ptr).size() / Layout::new::<T>().size();
    // casting prior to this provides the incorrect length ^
    let src_ptr = src_ptr as *const T;
    let dst_ptr = slice.as_mut_ptr().add(dst);

    ptr::copy(src_ptr, dst_ptr, src_len);
}
