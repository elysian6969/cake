use crate::{array, mem};
use core::mem::MaybeUninit;
use core::ops;

/// A wrapper around `[MaybeUninit<T>; N]`.
#[repr(transparent)]
pub struct MaybeUninitArray<T, const N: usize> {
    array: [MaybeUninit<T>; N],
}

impl<T, const N: usize> MaybeUninitArray<T, N> {
    #[inline]
    pub const fn uninit() -> Self {
        let array = MaybeUninit::uninit_array();

        Self { array }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.array.as_ptr().cast()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.array.as_mut_ptr().cast()
    }

    /// # Safety
    ///
    /// see Maybeuninit
    #[inline]
    pub const unsafe fn assume_init(array: Self) -> [T; N] {
        MaybeUninit::array_assume_init(array.array)
    }

    /// # Safety
    ///
    /// see Maybeuninit
    #[inline]
    pub const unsafe fn assume_init_ref(array: &Self) -> &[T; N] {
        mem::transmute_unchecked(array)
    }

    #[inline]
    pub const fn each_ref(array: &Self) -> [&MaybeUninit<T>; N] {
        array::each_ref(&array.array)
    }

    #[inline]
    pub const fn each_mut(array: &mut Self) -> [&mut MaybeUninit<T>; N] {
        array::each_mut(&mut array.array)
    }

    #[inline]
    pub const fn each_ptr(array: &Self) -> [*const T; N] {
        let array = MaybeUninitArray::each_ref(array);

        // SAFETY: MaybeUninit is repr(transparent)
        unsafe { mem::transmute_array(array) }
    }

    #[inline]
    pub const fn each_mut_ptr(array: &mut Self) -> [*mut T; N] {
        let array = MaybeUninitArray::each_mut(array);

        // SAFETY: MaybeUninit is repr(transparent)
        unsafe { mem::transmute_array(array) }
    }
}

impl<T, const N: usize> const ops::Deref for MaybeUninitArray<T, N> {
    type Target = [MaybeUninit<T>; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<T, const N: usize> const ops::DerefMut for MaybeUninitArray<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}
