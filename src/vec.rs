use crate::mem::UninitArray;

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
}
