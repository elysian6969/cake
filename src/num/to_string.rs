use crate::fixed::FixedString;
use crate::num;
use crate::num::{As, AsUnsigned, Int, One, Signed, ToChar, Unsigned, Zero};
use core::ops::{Add, Div, Rem};
pub use radix::{len, max_len, Radix};

mod radix;

#[inline]
const fn format_digits<T, const N: usize>(string: &mut FixedString<N>, mut value: T, radix: u8)
where
    T: Copy,
    T: ~const Add<Output = T>,
    T: ~const Div<Output = T>,
    T: ~const One,
    T: ~const Rem<Output = T>,
    T: ~const ToChar,
    T: ~const Unsigned,
    T: ~const Zero,
    u8: ~const As<T>,
{
    while !num::is_zero(value) {
        let (div, rem) = num::div_rem(value, num::cast(radix));
        let character = unsafe { num::to_char(rem, radix).unwrap_unchecked() };

        string.insert(0, character);

        value = div;
    }
}

#[inline]
const fn to_string_signed<T, U, const RADIX: u8>(value: T) -> FixedString<{ len::<T, RADIX>() }>
where
    T: ~const AsUnsigned<Output = U>,
    T: ~const Int,
    T: ~const PartialEq,
    T: ~const Radix,
    T: ~const Signed,
    U: Copy,
    U: ~const Add<Output = U>,
    U: ~const Div<Output = U>,
    U: ~const One,
    U: ~const Rem<Output = U>,
    U: ~const ToChar,
    U: ~const Unsigned,
    U: ~const Zero,
    u8: ~const As<U>,
{
    let mut string = FixedString::<{ len::<T, RADIX>() }>::new();
    let is_negative = <T as Signed>::is_negative(value);
    // handle T::MIN, as T::MIN.abs() would overflow
    let value = if value == <T as Int>::MIN {
        num::as_unsigned(value)
    } else {
        num::as_unsigned(<T as Signed>::abs(value))
    };

    format_digits(&mut string, value, RADIX);

    if is_negative && RADIX != 2 {
        string.insert(0, '-');
    }

    string
}

#[inline]
const fn to_dyn_string_signed<T, U>(value: T, radix: u8) -> FixedString<{ max_len::<T>() }>
where
    T: ~const AsUnsigned<Output = U>,
    T: ~const Int,
    T: ~const PartialEq,
    T: ~const Radix,
    T: ~const Signed,
    U: Copy,
    U: ~const Add<Output = U>,
    U: ~const Div<Output = U>,
    U: ~const One,
    U: ~const Rem<Output = U>,
    U: ~const ToChar,
    U: ~const Unsigned,
    U: ~const Zero,
    u8: ~const As<U>,
{
    let mut string = FixedString::<{ max_len::<T>() }>::new();
    let is_negative = <T as Signed>::is_negative(value);
    // handle T::MIN, as T::MIN.abs() would overflow
    let value = if value == <T as Int>::MIN {
        num::as_unsigned(value)
    } else {
        num::as_unsigned(<T as Signed>::abs(value))
    };

    format_digits(&mut string, value, radix);

    if is_negative && radix != 2 {
        string.insert(0, '-');
    }

    string
}

#[inline]
const fn to_string_unsigned<T, const RADIX: u8>(value: T) -> FixedString<{ len::<T, RADIX>() }>
where
    T: Copy,
    T: ~const Add<Output = T>,
    T: ~const Div<Output = T>,
    T: ~const One,
    T: ~const Radix,
    T: ~const Rem<Output = T>,
    T: ~const ToChar,
    T: ~const Unsigned,
    T: ~const Zero,
    u8: ~const As<T>,
{
    let mut string = FixedString::<{ len::<T, RADIX>() }>::new();

    format_digits(&mut string, value, RADIX);

    string
}

#[inline]
const fn to_dyn_string_unsigned<T>(value: T, radix: u8) -> FixedString<{ max_len::<T>() }>
where
    T: Copy,
    T: ~const Add<Output = T>,
    T: ~const Div<Output = T>,
    T: ~const One,
    T: ~const Radix,
    T: ~const Rem<Output = T>,
    T: ~const ToChar,
    T: ~const Unsigned,
    T: ~const Zero,
    u8: ~const As<T>,
{
    let mut string = FixedString::<{ max_len::<T>() }>::new();

    format_digits(&mut string, value, radix);

    string
}

/// Convert a string to an integer.
pub trait ToString: Radix {
    fn to_string<const RADIX: u8>(self) -> FixedString<{ len::<Self, RADIX>() }>
    where
        Self: ~const Radix;

    fn to_dyn_string(self, radix: u8) -> FixedString<{ max_len::<Self>() }>;
}

macro_rules! impl_signed {
    ($($ident:ident),*) => { $(
        impl const ToString for $ident {
            #[inline]
            fn to_string<const RADIX: u8>(self) -> FixedString<{ len::<Self, RADIX>() }>
            where
                Self: ~const Radix,
            {
                to_string_signed::<Self, _, RADIX>(self)
            }

            #[inline]
            fn to_dyn_string(self, radix: u8) -> FixedString<{ max_len::<Self>() }> {
                to_dyn_string_signed::<Self, _>(self, radix)
            }
        }
    )* }
}

macro_rules! impl_unsigned {
    ($($ident:ident),*) => { $(
        impl const ToString for $ident {
            #[inline]
            fn to_string<const RADIX: u8>(self) -> FixedString<{ len::<Self, RADIX>() }>
            where
                Self: ~const Radix,
            {
                to_string_unsigned::<Self, RADIX>(self)
            }

            #[inline]
            fn to_dyn_string(self, radix: u8) -> FixedString<{ max_len::<Self>() }> {
                to_dyn_string_unsigned::<Self>(self, radix)
            }
        }
    )* }
}

impl_signed! { i8, i16, i32, i64, i128, isize }
impl_unsigned! { u8, u16, u32, u64, u128, usize }
