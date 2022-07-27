pub trait Sealed: Sized {}

/// Convert a character to a digit.
pub trait FromChar: Sealed {
    fn from_char(character: char, radix: u8) -> Option<Self>;
}

macro_rules! impl_from_char {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const FromChar  for $ident {
            #[inline]
            fn from_char(character: char, radix: u8) -> Option<Self> {
                Some(character.to_digit(radix as u32)? as Self)
            }
        }
    )* }
}

impl_from_char! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}
