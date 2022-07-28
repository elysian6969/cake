use super::{Int, One, Signed, ToChar, Zero};
use crate::char::Chars;
use crate::fixed::FixedString;
use crate::mem::Layout;
use core::ops::{Add, AddAssign, DivAssign, Mul, Neg, Rem};

#[inline]
pub const fn signed_len<T>(radix: u8) -> usize
where
    T: ~const FromRadix,
    T: ~const Int,
    T: ~const Signed,
    T: ~const Add<Output = T>,
    T: ~const One,
{
    // abs(min) would overflow
    let min = <T as Int>::MIN + super::one();
    let abs = <T as Signed>::abs(min);
    let log = <T as Int>::log(abs, <T as FromRadix>::from_radix(radix));

    // this also includes `-`
    log.saturating_add(3) as usize
}

#[inline]
pub const fn unsigned_len<T>(radix: u8) -> usize
where
    T: ~const FromRadix,
    T: ~const Int,
    T: ~const One,
{
    let max = <T as Int>::MAX;
    let log = <T as Int>::log(max, <T as FromRadix>::from_radix(radix));

    log.saturating_add(1) as usize
}

pub trait Radix: Copy + Sized {
    const MAX_LEN: usize;

    fn len<const RADIX: u8>() -> usize;
}

#[inline]
pub const fn len<T, const RADIX: u8>() -> usize
where
    T: ~const Radix,
{
    <T as Radix>::len::<RADIX>()
}

#[inline]
pub const fn max_len<T>() -> usize
where
    T: Radix,
{
    <T as Radix>::MAX_LEN
}

macro_rules! impl_signed_radix {
    ($($ident:ident),*) => { $(
        impl const Radix for $ident {
            const MAX_LEN: usize = signed_len::<Self>(2);

            #[inline]
            fn len<const RADIX: u8>() -> usize {
                signed_len::<Self>(RADIX)
            }
        }
    )* }
}

macro_rules! impl_unsigned_radix {
    ($($ident:ident),*) => { $(
        impl const Radix for $ident {
            const MAX_LEN: usize = unsigned_len::<Self>(2);

            #[inline]
            fn len<const RADIX: u8>() -> usize {
                unsigned_len::<Self>(RADIX)
            }
        }
    )* }
}

impl_signed_radix! { i8, i16, i32, i64, i128, isize }
impl_unsigned_radix! { u8, u16, u32, u64, u128, usize }

pub trait FromRadix: Copy + Sized {
    fn from_radix(radix: u8) -> Self;
}

macro_rules! impl_from_radix {
    ($($ident:ident),*) => { $(
        impl const FromRadix for $ident {
            #[inline]
            fn from_radix(radix: u8) -> Self {
                radix as Self
            }
        }
    )* }
}

#[inline]
pub const fn from_radix<T>(radix: u8) -> T
where
    T: ~const FromRadix,
{
    <T as FromRadix>::from_radix(radix)
}

impl_from_radix! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}

#[inline]
pub const fn signed_to_string<T, const RADIX: u8>(int: T) -> FixedString<{ len::<T, RADIX>() }>
where
    T: ~const Add<Output = T>,
    T: ~const AddAssign,
    T: ~const DivAssign,
    T: ~const FromRadix,
    T: ~const Radix,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const Neg<Output = T>,
    T: ~const One,
    T: ~const PartialEq,
    T: ~const Rem<Output = T>,
    T: ~const Signed,
    T: ~const ToChar,
    T: ~const Zero,
{
    let mut string = FixedString::<{ len::<T, RADIX>() }>::new();
    let is_negative = <T as Signed>::is_negative(int);
    let mut digit = <T as Signed>::abs(int);

    while !super::is_zero(digit) {
        let character = digit % from_radix(RADIX);

        digit /= from_radix(RADIX);

        let character = unsafe { super::to_char(character, RADIX).unwrap_unchecked() };

        string.insert(0, character);
    }

    if is_negative && from_radix::<T>(RADIX) != super::one::<T>() + super::one() {
        string.insert(0, '-');
    }

    string
}

#[inline]
pub const fn dyn_signed_to_string<T>(int: T, radix: u8) -> FixedString<{ max_len::<T>() }>
where
    T: ~const Add<Output = T>,
    T: ~const AddAssign,
    T: ~const DivAssign,
    T: ~const FromRadix,
    T: ~const Radix,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const Neg<Output = T>,
    T: ~const One,
    T: ~const PartialEq,
    T: ~const Rem<Output = T>,
    T: ~const Signed,
    T: ~const ToChar,
    T: ~const Zero,
{
    let mut string = FixedString::<{ max_len::<T>() }>::new();
    let is_negative = <T as Signed>::is_negative(int);
    let mut digit = <T as Signed>::abs(int);

    while !super::is_zero(digit) {
        let character = digit % from_radix(radix);

        digit /= from_radix(radix);

        let character = unsafe { super::to_char(character, radix).unwrap_unchecked() };

        string.insert(0, character);
    }

    if is_negative && from_radix::<T>(radix) != super::one::<T>() + super::one() {
        string.insert(0, '-');
    }

    string
}

#[inline]
pub const fn unsigned_to_string<T, const RADIX: u8>(int: T) -> FixedString<{ len::<T, RADIX>() }>
where
    T: ~const Add<Output = T>,
    T: ~const AddAssign,
    T: ~const DivAssign,
    T: ~const FromRadix,
    T: ~const Radix,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const One,
    T: ~const Rem<Output = T>,
    T: ~const ToChar,
    T: ~const Zero,
{
    let mut string = FixedString::<{ len::<T, RADIX>() }>::new();
    let mut digit = int;

    while !super::is_zero(digit) {
        let character = digit % from_radix(RADIX);

        digit /= from_radix(RADIX);

        let character = unsafe { super::to_char(character, RADIX).unwrap_unchecked() };

        string.insert(0, character);
    }

    string
}

#[inline]
pub const fn dyn_unsigned_to_string<T>(int: T, radix: u8) -> FixedString<{ max_len::<T>() }>
where
    T: ~const Add<Output = T>,
    T: ~const AddAssign,
    T: ~const DivAssign,
    T: ~const FromRadix,
    T: ~const Radix,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const One,
    T: ~const Rem<Output = T>,
    T: ~const ToChar,
    T: ~const Zero,
{
    let mut string = FixedString::<{ max_len::<T>() }>::new();
    let mut digit = int;

    while !super::is_zero(digit) {
        let character = digit % from_radix(radix);

        digit /= from_radix(radix);

        let character = unsafe { super::to_char(character, radix).unwrap_unchecked() };

        string.insert(0, character);
    }

    string
}

pub trait Sealed: Sized {}

/// Convert a string to an integer.
pub trait ToString: Radix + Sealed {
    fn to_string<const RADIX: u8>(self) -> FixedString<{ len::<Self, RADIX>() }>
    where
        Self: ~const Radix;

    fn to_dyn_string(self, radix: u8) -> FixedString<{ max_len::<Self>() }>;
}

macro_rules! impl_signed {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const ToString for $ident {
            #[inline]
            fn to_string<const RADIX: u8>(self) -> FixedString<{ len::<Self, RADIX>() }>
            where
                Self: ~const Radix,
            {
                signed_to_string::<_, RADIX>(self)
            }

            #[inline]
            fn to_dyn_string(self, radix: u8) -> FixedString<{ max_len::<Self>() }> {
                dyn_signed_to_string::<_>(self, radix)
            }
        }
    )* }
}

macro_rules! impl_unsigned {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const ToString for $ident {
            #[inline]
            fn to_string<const RADIX: u8>(self) -> FixedString<{ len::<Self, RADIX>() }>
            where
                Self: ~const Radix,
            {
                unsigned_to_string::<_, RADIX>(self)
            }

            #[inline]
            fn to_dyn_string(self, radix: u8) -> FixedString<{ max_len::<Self>() }> {
                dyn_unsigned_to_string::<_>(self, radix)
            }
        }
    )* }
}

impl_signed! { i8, i16, i32, i64, i128, isize }
impl_unsigned! { u8, u16, u32, u64, u128, usize }
