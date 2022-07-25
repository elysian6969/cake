use crate::mem::UninitArray;
use core::fmt;

pub type BytePad<const N: usize> = Pad<u8, N>;
pub type VTablePad<const N: usize> = Pad<unsafe extern "C" fn(), N>;

#[repr(C)]
pub struct Pad<T, const N: usize> {
    array: UninitArray<T, N>,
}

impl<T, const N: usize> Pad<T, N> {
    #[inline]
    pub const fn uninit() -> Self {
        let array = UninitArray::uninit();

        Self { array }
    }
}

impl<T, const N: usize> fmt::Debug for Pad<T, N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("Pad {{ {N} elements }}"))
    }
}
