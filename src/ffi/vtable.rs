use core::fmt;
use core::mem::MaybeUninit;

#[repr(C)]
pub struct Pad<const N: usize>([MaybeUninit<unsafe extern "C" fn()>; N]);

impl<const N: usize> Pad<N> {
    #[inline]
    pub const fn uninit() -> Self {
        Self(MaybeUninit::uninit_array())
    }
}

impl<const N: usize> fmt::Debug for Pad<N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("Pad {{ {N} virtual methods }}"))
    }
}
