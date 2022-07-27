use super::{One, Zero};
use crate::mem::Layout;
use core::ops::{Add, Div, Mul, Rem, Sub};
use core::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

pub trait Sealed {}

/// An integer.
pub trait Int:
    Copy
    + Sized
    + Eq
    + Ord
    + PartialEq
    + PartialOrd
    + Add
    + Div
    + Mul
    + Rem
    + Sub
    + AddAssign
    + DivAssign
    + MulAssign
    + RemAssign
    + SubAssign
    + One
    + Zero
{
    #[doc(hidden)]
    fn _cake_base(offset: usize, radix: u8) -> Self;

    #[doc(hidden)]
    fn _cake_from_be_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self;

    #[doc(hidden)]
    fn _cake_from_le_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self;

    #[doc(hidden)]
    fn _cake_from_ne_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self;
}

macro_rules! impl_int {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const Int for $ident {
            #[doc(hidden)]
            fn _cake_base(offset: usize, radix: u8) -> Self {
                (radix as Self).pow(offset as u32)
            }

            #[inline]
            fn _cake_from_be_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self
            where
                [(); Layout::new::<Self>().size()]:,
            {
                Self::from_be_bytes(bytes)
            }

            #[inline]
            fn _cake_from_le_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self
            where
                [(); Layout::new::<Self>().size()]:,
            {
                Self::from_le_bytes(bytes)
            }

            #[inline]
            fn _cake_from_ne_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self
            where
                [(); Layout::new::<Self>().size()]:,
            {
                Self::from_ne_bytes(bytes)
            }
        }
    )* }
}

impl_int! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}
