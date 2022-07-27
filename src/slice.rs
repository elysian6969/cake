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
#[inline]
pub const fn copy_within<T, I>(slice: &mut [T], src: I, dst: usize)
where
    T: Copy,
    I: ~const SliceIndex<[T]>,
{
    if dst > slice.len() {
        panic!("dst out of bounds");
    }

    copy_within_unchecked(slice, src, dst)
}

/// Copies elements from one part of the slice to another part of itself.
///
/// # Safety
///
/// Be sure to properly drop elements as this does a bitwise copy.
#[inline]
pub const fn copy_within_unchecked<T, I>(slice: &mut [T], src: I, dst: usize)
where
    I: ~const SliceIndex<[T]>,
{
    // `SliceIndex::get_unchecked` always returns a reference
    let src = unsafe { slice.get_unchecked(src) };
    // compiler should return the correct size (in bytes) based on the type
    // if `src` is `&T` it should be `size_of::<T>()`
    // if `src` is `&[T]` it should be `src.len() * size_of::<T>()`
    let src_len = Layout::from_val(src).size() / Layout::new::<T>().size();
    // it is safe to cast a slice to a fat pointer then to a regular pointer
    // fat pointer repr (*const T, usize)
    let src_ptr = src as *const _ as *const T;
    let dst_ptr = unsafe { slice.as_mut_ptr().add(dst) };
    // permit `copy_within_unchecked(slice, 1.., 5)`.
    let max_len = slice.len().saturating_sub(dst);
    let src_len = src_len.min(max_len);

    unsafe {
        ptr::copy(src_ptr, dst_ptr, src_len);
    }
}

#[inline]
pub const unsafe fn insert_slice_unchecked<T>(dst: &mut [T], src: &[T], index: usize) {
    //let dst_len = dst.len();
    let src_len = src.len();

    copy_within_unchecked(dst, index.., index + src_len);
    ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().add(index), src_len);
}

#[inline]
pub const fn to_raw_parts<'a, T>(slice: &'a [T]) -> (*const T, usize) {
    (slice.as_ptr(), slice.len())
}

#[inline]
pub const fn to_raw_parts_mut<'a, T>(slice: &'a mut [T]) -> (*mut T, usize) {
    (slice.as_mut_ptr(), slice.len())
}
