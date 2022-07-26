use crate::mem::UninitArray;
use core::{fmt, ops, ptr, slice};

/// A fixed-capacity vector type.
pub struct FixedVec<T, const N: usize> {
    array: UninitArray<T, N>,
    len: usize,
}

impl<T, const N: usize> FixedVec<T, N> {
    #[inline]
    pub const fn new() -> Self {
        let array = UninitArray::uninit();
        let len = 0;

        Self { array, len }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub const unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Appends an element to the back of a collection.
    ///
    /// # Panics
    ///
    /// Panics if the length exceeds `N` elements.
    #[inline]
    pub const fn push(&mut self, value: T) {
        self.array[self.len].write(value);
        self.len += 1;
    }

    #[inline]
    pub const fn extend_from_slice(&mut self, slice: &[T]) {
        unsafe {
            let address = slice.as_ptr();
            let base = self.as_mut_ptr().add(self.len);
            let len = slice.len();
            let new_len = self.len + len;

            if new_len > N {
                panic!("overflows capacity");
            }

            ptr::copy_nonoverlapping(address, base, len);

            self.len = new_len;
        }
    }

    /// Removes the last element from the collection  and returns it, or `None` if it is empty.
    #[inline]
    pub const fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;

            let value = unsafe { UninitArray::take_init(&mut self.array, self.len) };

            Some(value)
        }
    }

    #[inline]
    pub const fn as_ptr(&self) -> *const T {
        self.array.as_ptr().cast()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut T {
        self.array.as_mut_ptr().cast()
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

impl<T, const N: usize> fmt::Debug for FixedVec<T, N>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_slice(), fmt)
    }
}

impl<T, const N: usize> const ops::Deref for FixedVec<T, N> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> const ops::DerefMut for FixedVec<T, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
