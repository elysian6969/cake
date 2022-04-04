use crate::array;
use crate::array::Array;
use crate::tuple::{FromTupleRef, Tuple};
use core::ops::Range;
use core::slice;

mod sealed {
    pub trait Sealed {}
}

pub trait Slice: sealed::Sealed {}

pub unsafe trait SliceIndex<T>
where
    T: ?Sized,
{
    type Output: ?Sized;

    fn get(self, slice: &T) -> Option<&Self::Output>;
    fn get_mut(self, slice: &mut T) -> Option<&mut Self::Output>;

    unsafe fn get_unchecked(self, slice: *const T) -> *const Self::Output;
    unsafe fn get_unchecked_mut(self, slice: *mut T) -> *mut Self::Output;
}

impl<T> const sealed::Sealed for &[T] {}
impl<T> const Slice for &[T] {}

unsafe impl<T> const SliceIndex<[T]> for usize {
    type Output = T;

    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        if self > slice.len() {
            None
        } else {
            Some(unsafe { &*Self::get_unchecked(self, slice) })
        }
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        if self > slice.len() {
            None
        } else {
            Some(unsafe { &mut *Self::get_unchecked_mut(self, slice) })
        }
    }

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        (slice as *const T).add(self)
    }

    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        (slice as *mut T).add(self)
    }
}

unsafe impl<T> const SliceIndex<[T]> for Range<usize> {
    type Output = [T];

    fn get(self, slice: &[T]) -> Option<&Self::Output> {
        let Range { start, end } = self;

        if end > slice.len() || start > end {
            None
        } else {
            Some(unsafe { &*Self::get_unchecked(self, slice) })
        }
    }

    fn get_mut(self, slice: &mut [T]) -> Option<&mut Self::Output> {
        let Range { start, end } = self;

        if end > slice.len() || start > end {
            None
        } else {
            Some(unsafe { &mut *Self::get_unchecked_mut(self, slice) })
        }
    }

    unsafe fn get_unchecked(self, slice: *const [T]) -> *const Self::Output {
        let Range { start, end } = self;

        slice::from_raw_parts((slice as *mut T).add(start), end.saturating_sub(start))
    }

    unsafe fn get_unchecked_mut(self, slice: *mut [T]) -> *mut Self::Output {
        let Range { start, end } = self;

        slice::from_raw_parts_mut((slice as *mut T).add(start), end.saturating_sub(start))
    }
}

pub const fn get<T, I>(slice: &[T], index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
where
    I: ~const SliceIndex<[T]>,
{
    <I as SliceIndex<[T]>>::get(index, slice)
}

pub const fn get_mut<T, I>(slice: &mut [T], index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
where
    I: ~const SliceIndex<[T]>,
{
    <I as SliceIndex<[T]>>::get_mut(index, slice)
}

pub const unsafe fn get_unchecked<T, I>(slice: &[T], index: I) -> &<I as SliceIndex<[T]>>::Output
where
    I: ~const SliceIndex<[T]>,
{
    &*<I as SliceIndex<[T]>>::get_unchecked(index, slice)
}

pub const unsafe fn get_unchecked_mut<T, I>(
    slice: &mut [T],
    index: I,
) -> &mut <I as SliceIndex<[T]>>::Output
where
    I: ~const SliceIndex<[T]>,
{
    &mut *<I as SliceIndex<[T]>>::get_unchecked_mut(index, slice)
}

/// Converts a reference to a tuple of `T` to an array with an equivalent length, without copying.
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as FromTupleRef>::Element]
where
    T: ~const FromTupleRef,
    <T as FromTupleRef>::Output: ~const Array<<T as FromTupleRef>::Element>,
    [(); <T as Tuple>::LEN]:,
{
    Array::as_slice(array::from_tuple_ref(tuple))
}
