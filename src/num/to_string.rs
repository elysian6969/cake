use super::{Int, One, Signed, ToChar, Zero};
use crate::char::Chars;
use crate::fixed::FixedString;
use crate::mem::Layout;
use core::ops::{Add, AddAssign, DivAssign, Mul, Neg, Rem};

#[inline]
pub const fn digits_for<T, const RADIX: u8>() -> usize
where
    T: ~const FromRadix,
    T: ~const Int,
    T: ~const Signed,
    T: ~const Add<Output = T>,
    T: ~const One,
{
    let min = <T as Int>::MIN + <T as One>::one();
    let abs = <T as Signed>::abs(min);
    let log = <T as Int>::log(abs, <T as FromRadix>::from_radix(RADIX));

    log.saturating_add(2) as usize
}

pub trait FromRadix: Copy + Sized {
    fn from_radix(radix: u8) -> Self;
}

impl const FromRadix for i32 {
    fn from_radix(radix: u8) -> Self {
        radix as Self
    }
}

#[inline]
pub const fn signed_to_string<T, const RADIX: u8>(
    int: T,
) -> FixedString<{ digits_for::<T, RADIX>() }>
where
    T: ~const Add<Output = T>,
    T: ~const AddAssign,
    T: ~const DivAssign,
    T: ~const FromRadix,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const Neg<Output = T>,
    T: ~const One,
    T: ~const Rem<Output = T>,
    T: ~const Signed,
    T: ~const ToChar,
    T: ~const Zero,
{
    let mut string = FixedString::<{ digits_for::<T, RADIX>() }>::new();
    let is_negative = <T as Signed>::is_negative(int);
    let mut digit = <T as Signed>::abs(int);

    while !<T as Zero>::is_zero(digit) {
        let character = digit % <T as FromRadix>::from_radix(RADIX);

        digit /= <T as FromRadix>::from_radix(RADIX);

        let character = unsafe { <T as ToChar>::to_char(character, RADIX).unwrap_unchecked() };

        string.insert(0, character);
    }

    string
}

/*pub trait Sealed: Sized {}

/// Convert a string to an integer.
pub trait ToString: Sealed {
    fn to_string<const RADIX: u8>(self) -> Option<FixedString<{ digits_for::<Self, RADIX>() }>>;
    //fn to_variable_string(self, radix: u8) -> Option<FixedString<{ digits_for::<Self, 2>() }>>;
}

macro_rules! impl_signed {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const ToString for $ident {
            #[inline]
            fn to_string<const RADIX: u8>(self) ->
                Option<FixedString<{ digits_for::<Self, RADIX>() }>> {
                signed_to_string::<Self, RADIX>(self)
            }
        }
    )* }
}

macro_rules! impl_unsigned {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const ToString for $ident {
            #[inline]
            fn to_string(string: &str, radix: u8) -> Option<Self> {
                unsigned_to_string(string, radix)
            }
        }
    )* }
}

impl_signed! { i8, i16, i32, i64, i128, isize }*/
//impl_unsigned! { u8, u16, u32, u64, u128, usize }
