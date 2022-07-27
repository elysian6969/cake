pub trait Sealed: Sized {}

/// Convert a digit to a character.
pub trait ToChar: Sealed {
    fn to_char(self, radix: u8) -> Option<char>;
}

macro_rules! impl_to_char {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const ToChar  for $ident {
            #[inline]
            fn to_char(self, radix: u8) -> Option<char> {
                char::from_digit(self as u32, radix as u32)
            }
        }
    )* }
}

impl_to_char! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}
