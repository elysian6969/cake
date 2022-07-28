use crate::fixed::FixedString;
use crate::mem;
use crate::mem::Layout;
use crate::traits::{Assert, True};

pub use float::Float;
pub use from_char::FromChar;
pub use from_str::FromStr;
pub use identity::{is_one, is_zero, one, zero, One, Zero};
pub use int::Int;
pub use signed::Signed;
pub use to_char::ToChar;
pub use to_string::{len, max_len, FromRadix, Radix, ToString};

mod float;
mod from_char;
mod from_str;
mod identity;
mod int;
mod signed;
mod to_char;
#[allow(warnings)]
pub mod to_string;

/// Create a native endian integer value from its representation as a byte array in big endian.
#[inline]
pub const fn from_be_bytes<T: ~const Int>(bytes: [u8; Layout::new::<T>().size()]) -> T
where
    [(); Layout::new::<T>().size()]:,
{
    <T as Int>::_cake_from_be_bytes(bytes)
}

/// Create a native endian integer value from its representation as a byte array in little endian.
#[inline]
pub const fn from_le_bytes<T: ~const Int>(bytes: [u8; Layout::new::<T>().size()]) -> T
where
    [(); Layout::new::<T>().size()]:,
{
    <T as Int>::_cake_from_le_bytes(bytes)
}

/// Create an integer value from its memory representation as a byte array in native endianness.
#[inline]
pub const fn from_ne_bytes<T: ~const Int>(bytes: [u8; Layout::new::<T>().size()]) -> T
where
    [(); Layout::new::<T>().size()]:,
{
    <T as Int>::_cake_from_ne_bytes(bytes)
}

// based on https://github.com/rust-lang/rust/blob/master/library/core/src/num/mod.rs#L902
/// Returns a `T` where every byte is set to `byte`.
#[inline]
pub const fn repeat_u8<T: ~const Int>(byte: u8) -> T
where
    [(); Layout::new::<T>().size()]:,
{
    from_ne_bytes([byte; Layout::new::<T>().size()])
}

/// Returns a `T` where every two bytes is set to `bytes`.
#[inline]
pub const fn repeat_u16<T: ~const Int>(bytes: u16) -> T
where
    Assert<{ Layout::new::<T>().size() >= 2 }>: True,
    [(); Layout::new::<T>().size()]:,
    [(); Layout::new::<T>().size() / 2]:,
{
    let bytes = [bytes; Layout::new::<T>().size() / 2];
    let bytes = unsafe { mem::transmute_unchecked(bytes) };

    from_ne_bytes(bytes)
}

/// Returns a `T` where every four bytes is set to `bytes`.
#[inline]
pub const fn repeat_u32<T: ~const Int>(bytes: u32) -> T
where
    Assert<{ Layout::new::<T>().size() >= 4 }>: True,
    [(); Layout::new::<T>().size()]:,
    [(); Layout::new::<T>().size() / 4]:,
{
    let bytes = [bytes; Layout::new::<T>().size() / 4];
    let bytes = unsafe { mem::transmute_unchecked(bytes) };

    from_ne_bytes(bytes)
}

/// Returns a `T` where every four bytes is set to `bytes`.
#[inline]
pub const fn repeat_u64<T: ~const Int>(bytes: u64) -> T
where
    Assert<{ Layout::new::<T>().size() >= 8 }>: True,
    [(); Layout::new::<T>().size()]:,
    [(); Layout::new::<T>().size() / 8]:,
{
    let bytes = [bytes; Layout::new::<T>().size() / 8];
    let bytes = unsafe { mem::transmute_unchecked(bytes) };

    from_ne_bytes(bytes)
}

const FRACTION: f64 = 1.0;

/// Calculates the root of a value.
#[inline]
pub const fn root(value: i32, root: u32) -> i32 {
    libm::round(libm::pow(value as f64, FRACTION / root as f64)) as i32
}

/// Convert a character to a digit.
#[inline]
pub const fn from_char<T>(character: char, radix: u8) -> Option<T>
where
    T: ~const FromChar,
{
    <T as FromChar>::from_char(character, radix)
}

/// Convert a string to an integer.
#[inline]
pub const fn from_str<T>(string: &str, radix: u8) -> Option<T>
where
    T: ~const FromStr,
{
    <T as FromStr>::from_str(string, radix)
}

/// Convert a digit to a character.
#[inline]
pub const fn to_char<T>(digit: T, radix: u8) -> Option<char>
where
    T: ~const ToChar,
{
    <T as ToChar>::to_char(digit, radix)
}

/// Convert an integer to a string.
#[inline]
pub const fn to_string<T, const RADIX: u8>(digit: T) -> FixedString<{ len::<T, RADIX>() }>
where
    T: ~const ToString,
    T: ~const Radix,
{
    <T as ToString>::to_string::<RADIX>(digit)
}

/// Convert an integer to a string.
#[inline]
pub const fn to_dyn_string<T>(digit: T, radix: u8) -> FixedString<{ max_len::<T>() }>
where
    T: ~const ToString,
{
    <T as ToString>::to_dyn_string(digit, radix)
}
