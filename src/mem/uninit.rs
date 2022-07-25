use crate::{array, mem};
use core::mem::MaybeUninit;
use core::{fmt, ops};

/// A wrapper around `[MaybeUninit<T>; N]`.
#[repr(transparent)]
pub struct UninitArray<T, const N: usize> {
    array: [MaybeUninit<T>; N],
}

impl<T, const N: usize> UninitArray<T, N> {
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
        let array = UninitArray::each_ref(array);

        // SAFETY: MaybeUninit is repr(transparent)
        unsafe { mem::transmute_array_unchecked(array) }
    }

    #[inline]
    pub const fn each_mut_ptr(array: &mut Self) -> [*mut T; N] {
        let array = UninitArray::each_mut(array);

        // SAFETY: MaybeUninit is repr(transparent)
        unsafe { mem::transmute_array_unchecked(array) }
    }
}

impl<T, const N: usize> const ops::Deref for UninitArray<T, N> {
    type Target = [MaybeUninit<T>; N];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.array
    }
}

impl<T, const N: usize> const ops::DerefMut for UninitArray<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.array
    }
}

impl<T, const N: usize> fmt::Debug for UninitArray<T, N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.array, fmt)
    }
}
