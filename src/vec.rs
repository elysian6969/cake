use core::mem::MaybeUninitArray;

pub struct FixedVec<T, const N: usize> {
    array: MaybeUninitArray<T, N>,
    len: usize,
}

impl<T, const N: usize> FixedVec<T, N> {
    pub const fn new() -> Self {
        let array = MaybeUninitArray::uninit();
        let len = 0;

        Self { array, len }
    }
}
