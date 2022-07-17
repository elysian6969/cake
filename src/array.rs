//! Helper functions and types for arrays.

use crate::mem;
use crate::tuple::{Tuple, TupleArray};
use core::mem::MaybeUninit;

#[inline]
pub const fn from_tuple<T>(tuple: T) -> [<T as TupleArray>::Element; <T as Tuple>::LEN]
where
    T: ~const TupleArray,
    [(); <T as Tuple>::LEN]:,
{
    // SAFETY: trait bounds ensure this is valid
    unsafe { mem::transmute(tuple) }
}

#[inline]
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as TupleArray>::Element; <T as Tuple>::LEN]
where
    T: ~const TupleArray,
    [(); <T as Tuple>::LEN]:,
{
    let array =
        unsafe { &*(tuple as *const T as *const [<T as TupleArray>::Element; <T as Tuple>::LEN]) };

    array
}

#[inline]
pub const fn from_tuple_mut<T>(
    tuple: &mut T,
) -> &mut [<T as TupleArray>::Element; <T as Tuple>::LEN]
where
    T: ~const TupleArray,
    [(); <T as Tuple>::LEN]:,
{
    let array =
        unsafe { &mut *(tuple as *mut T as *mut [<T as TupleArray>::Element; <T as Tuple>::LEN]) };

    array
}

#[inline]
pub const fn fold<'a, F, D, T, const N: usize>(data: D, mut f: F) -> [T; N]
where
    D: Copy,
    F: Copy,
    F: ~const FnMut(D, usize) -> T,
{
    let mut new_array = MaybeUninit::uninit_array();
    let mut index = 0;

    while index < N {
        let item = f(data, index);

        new_array[index] = MaybeUninit::new(item);

        index += 1;
    }

    unsafe { MaybeUninit::array_assume_init(new_array) }
}

#[inline]
const fn fold_ref<T, const N: usize>(array: &[T; N], index: usize) -> &T {
    unsafe { array.get_unchecked(index) }
}

#[inline]
pub const fn each_ref<T, const N: usize>(array: &[T; N]) -> [&T; N] {
    fold(array, fold_ref)

    // when const closures exist
    // from_fn(|index| unsafe { array.get_unchecked(index) })
}

// this uses `*mut [T; N]` since `&mut [T; N]` is not copyable
#[inline]
const fn fold_mut<'a, T, const N: usize>(array: *mut [T; N], index: usize) -> &'a mut T {
    unsafe { (*array).get_unchecked_mut(index) }
}

#[inline]
pub const fn each_mut<T, const N: usize>(array: &mut [T; N]) -> [&mut T; N] {
    fold(array as *mut [T; N], fold_mut)
}

#[inline]
pub const fn each_as_ptr<T, const N: usize>(array: [&T; N]) -> [*const T; N] {
    unsafe { mem::transmute_array(array) }
}

#[inline]
pub const fn each_as_mut_ptr<T, const N: usize>(array: [&mut T; N]) -> [*mut T; N] {
    unsafe { mem::transmute_array(array) }
}
