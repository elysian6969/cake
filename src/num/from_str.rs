use super::{FromChar, Int, One, Zero};
use crate::char::Chars;
use core::ops::{AddAssign, Mul, Neg, Range};

#[inline]
const fn apply<T>(character: char, radix: u8, offset: usize, value: &mut T) -> Option<()>
where
    T: ~const AddAssign,
    T: ~const FromChar,
    T: ~const Int,
    T: ~const Mul<Output = T>,
{
    let digit =
        <T as FromChar>::from_char(character, radix)? * <T as Int>::_cake_base(offset, radix);

    *value += digit;

    Some(())
}

#[inline]
const fn in_range(range: Range<char>, character: char) -> bool {
    character >= range.start && character <= range.end
}

#[inline]
pub const fn signed_from_str<T>(string: &str, radix: u8) -> Option<T>
where
    T: ~const AddAssign,
    T: ~const FromChar,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const Neg<Output = T>,
    T: ~const One,
    T: ~const Zero,
{
    let mut chars = Chars::new(string);
    let mut value = <T as Zero>::zero();
    let character = chars.next()?;
    let range = '0'..(b'0' + radix as u8) as char;
    let mut sign = <T as One>::one();
    let mut offset = 0;

    match character {
        '+' => {}
        '-' => sign = -<T as One>::one(),
        character if in_range(range, character) => {
            apply(character, radix, 0, &mut value)?;
            offset += 1;
        }
        _ => return None,
    }

    while let Some(character) = chars.next() {
        apply(character, radix, offset, &mut value)?;
        offset += 1;
    }

    Some(sign * value)
}

#[inline]
pub const fn unsigned_from_str<T>(string: &str, radix: u8) -> Option<T>
where
    T: ~const AddAssign,
    T: ~const FromChar,
    T: ~const Int,
    T: ~const Mul<Output = T>,
    T: ~const Zero,
{
    let mut chars = Chars::new(string);
    let mut value = <T as Zero>::zero();
    let character = chars.next()?;
    let range = '0'..(b'0' + radix as u8) as char;

    if in_range(range, character) {
        apply(character, radix, 0, &mut value)?;
    } else {
        return None;
    }

    let mut offset = 1;

    while let Some(character) = chars.next() {
        apply(character, radix, offset, &mut value)?;
        offset += 1;
    }

    Some(value)
}

pub trait Sealed: Sized {}

/// Convert a string to an integer.
pub trait FromStr: Sealed {
    fn from_str(string: &str, radix: u8) -> Option<Self>;
}

macro_rules! impl_signed {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const FromStr for $ident {
            #[inline]
            fn from_str(string: &str, radix: u8) -> Option<Self> {
                signed_from_str(string, radix)
            }
        }
    )* }
}

macro_rules! impl_unsigned {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const FromStr for $ident {
            #[inline]
            fn from_str(string: &str, radix: u8) -> Option<Self> {
                unsigned_from_str(string, radix)
            }
        }
    )* }
}

impl_signed! { i8, i16, i32, i64, i128, isize }
impl_unsigned! { u8, u16, u32, u64, u128, usize }
