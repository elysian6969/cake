use crate::mem::UninitArray;
use core::mem::MaybeUninit;

/// A fixed-capacity vector type.
pub struct FixedVec<T, const N: usize> {
    array: UninitArray<T, N>,
    len: usize,
}

impl<T, const N: usize> FixedVec<T, N> {
    pub const fn new() -> Self {
        let array = UninitArray::uninit();
        let len = 0;

        Self { array, len }
    }

    pub const fn len(&self) -> usize {
        self.len
    }

    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub const fn push(&mut self, value: T) -> Result<(), T> {
        if self.len > N {
            Err(value)
        } else {
            unsafe {
                self.push_unchecked(value);
            }

            Ok(())
        }
    }

    pub const unsafe fn push_unchecked(&mut self, value: T) {
        self.array[self.len] = MaybeUninit::new(value);
        self.len += 1;
    }
}
