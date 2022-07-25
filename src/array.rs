//! Helper functions and types for arrays.

use crate::mem;
use crate::mem::UninitArray;
use crate::tuple;
use core::mem::MaybeUninit;

/// Folds every element into an accumulator by applying an operation, returning an array of all the
/// results.
#[inline]
pub const fn fold<'a, F, D, T, const N: usize>(data: D, mut f: F) -> [T; N]
where
    D: Copy,
    F: Copy,
    F: ~const FnMut(D, usize) -> T,
{
    let mut array = UninitArray::uninit();
    let mut index = 0;

    while index < N {
        let item = f(data, index);
        array[index] = MaybeUninit::new(item);
        index += 1;
    }

    // SAFETY: all elements have been initialized by the loop
    unsafe { UninitArray::assume_init(array) }
}

/// Convert a contiguous tuple to an array.
#[inline]
pub const fn from_tuple<T>(tuple: T) -> [<T as tuple::Array>::Element; tuple::len::<T>()]
where
    T: tuple::Array,
    [(); tuple::len::<T>()]:,
{
    // SAFETY: Trait bounds ensure this is never invalid.
    unsafe { mem::transmute_unchecked(tuple) }
}

/// Convert a contiguous tuple reference to an array reference.
#[inline]
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as tuple::Array>::Element; tuple::len::<T>()]
where
    T: tuple::Array,
    [(); tuple::len::<T>()]:,
{
    // SAFETY: Trait bounds ensure this is never invalid.
    unsafe { &*(tuple as *const T).cast() }
}

/// Convert a mutable contiguous tuple reference to an array reference.
#[inline]
pub const fn from_tuple_mut<T>(
    tuple: &mut T,
) -> &mut [<T as tuple::Array>::Element; tuple::len::<T>()]
where
    T: tuple::Array,
    [(); tuple::len::<T>()]:,
{
    // SAFETY: Trait bounds ensure this is never invalid.
    unsafe { &mut *(tuple as *mut T).cast() }
}

// each_ref fold operation
#[inline]
const fn fold_ref<T, const N: usize>(array: &[T; N], index: usize) -> &T {
    // SAFETY: `index` is always valid as this is called `N` times.
    unsafe { array.get_unchecked(index) }
}

// each_mut fold operation
// SAFETY: this uses `*mut [T; N]` since `&mut [T; N]` is not copyable
#[inline]
const fn fold_mut<'a, T, const N: usize>(array: *mut [T; N], index: usize) -> &'a mut T {
    // SAFETY: `index` is always valid as this is called `N` times.
    unsafe { (*array).get_unchecked_mut(index) }
}

// NOTE: const closures when?
// `from_fn(|index| unsafe { array.get_unchecked(index) })`

/// Borrows each element and returns an array of references with the same size as `array`.
#[inline]
pub const fn each_ref<T, const N: usize>(array: &[T; N]) -> [&T; N] {
    fold(array, fold_ref)
}

/// Borrows each element mutably and returns an array of references with the same size as `array`.
#[inline]
pub const fn each_mut<T, const N: usize>(array: &mut [T; N]) -> [&mut T; N] {
    fold(array as *mut [T; N], fold_mut)
}

/// Coerces an array of references to an array of pointers.
#[inline]
pub const fn each_as_ptr<T, const N: usize>(array: [&T; N]) -> [*const T; N] {
    // SAFETY: nothing special, &T coerces to *const T
    unsafe { mem::transmute_array_unchecked(array) }
}

/// Coerces an array of mutable references to an array of pointers.
#[inline]
pub const fn each_as_mut_ptr<T, const N: usize>(array: [&mut T; N]) -> [*mut T; N] {
    // SAFETY: nothing special, &mut T coerces to *mut T
    unsafe { mem::transmute_array_unchecked(array) }
}
