use core::mem::UninitArray;

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
}
