use crate::{array, mem};
use core::mem::replace;
use core::mem::MaybeUninit;
use core::{fmt, ops};

/// A wrapper around `[MaybeUninit<T>; N]`.
#[repr(transparent)]
pub struct UninitArray<T, const N: usize> {
    array: [MaybeUninit<T>; N],
}

impl<T, const N: usize> UninitArray<T, N> {
    const UNINIT: MaybeUninit<T> = MaybeUninit::uninit();

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

    /// Extracts the values from an array of MaybeUninit containers.
    #[inline]
    pub const unsafe fn assume_init(array: Self) -> [T; N] {
        MaybeUninit::array_assume_init(array.array)
    }

    /// Assuming all the elements are initialized, get an array reference to them.
    #[inline]
    pub const unsafe fn assume_init_ref(array: &Self) -> &[T; N] {
        // SAFETY: UninitArray is repr(transparent)
        unsafe { &*(array as *const Self as *const [T; N]) }
    }

    /// Assuming all the elements are initialized, get a mutable array reference to them.
    #[inline]
    pub const unsafe fn assume_init_mut(array: &mut Self) -> &mut [T; N] {
        // SAFETY: UninitArray is repr(transparent)
        unsafe { &mut *(array as *mut Self as *mut [T; N]) }
    }

    /// Borrows each element and returns an array of references with the same size as `self`.
    ///
    /// Equivalent to `array::each_ref` on the underlying `[MaybeUninit<T>; N]`.
    #[inline]
    pub const fn each_ref(array: &Self) -> [&MaybeUninit<T>; N] {
        // NOTE: this turns an array of elements into an array of references
        array::each_ref(&array.array)
    }

    /// Borrows each element mutably and returns an array of references with the same size as `self`.
    ///
    /// Equivalent to `array::each_mut` on the underlying `[MaybeUninit<T>; N]`.
    #[inline]
    pub const fn each_mut(array: &mut Self) -> [&mut MaybeUninit<T>; N] {
        // NOTE: this turns an array of elements into an array of references
        array::each_mut(&mut array.array)
    }

    /// Borrows each element and returns an array of pointers with the same size as `self`.
    #[inline]
    pub const fn each_ptr(array: &Self) -> [*const T; N] {
        let array = UninitArray::each_ref(array);

        // NOTE: we do a conversion, hence no use of `array::each_ptr`.
        // SAFETY: MaybeUninit is repr(transparent)
        unsafe { mem::transmute_array_unchecked(array) }
    }

    /// Borrows each element mutably and returns an array of pointers with the same size as `self`.
    #[inline]
    pub const fn each_mut_ptr(array: &mut Self) -> [*mut T; N] {
        let array = UninitArray::each_mut(array);

        // NOTE: we do a conversion, hence no use of `array::each_mut_ptr`.
        // SAFETY: MaybeUninit is repr(transparent)
        unsafe { mem::transmute_array_unchecked(array) }
    }

    /// Extracts the value at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than `N`.
    ///
    /// # Safety
    ///
    /// Caller must enaure item at `index` is initialised.
    #[inline]
    pub const unsafe fn take_init(array: &mut Self, index: usize) -> T {
        let value = replace(&mut array.array[index], Self::UNINIT);

        // SAFETY: `index` is checked.
        unsafe { MaybeUninit::assume_init(value) }
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
