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
    // `SliceIndex::get_unchecked` always returns a reference
    let src = slice.get_unchecked(src);
    // compiler should return the correct size (in bytes) based on the type
    // if `src` is `&T` it should be `size_of::<T>()`
    // if `src` is `&[T]` it should be `src.len() * size_of::<T>()`
    let src_len = Layout::from_val(src).size() / Layout::new::<T>().size();
    // it is safe to cast a slice to a fat pointer then to a regular pointer
    // fat pointer repr (*const T, usize)
    let src_ptr = src_ptr as *const _ as *const T;
    let dst_ptr = slice.as_mut_ptr().add(dst);

    ptr::copy(src_ptr, dst_ptr, src_len);
}
