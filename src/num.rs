use crate::mem;
use crate::mem::Layout;
use crate::traits::{Assert, True};

pub use float::Float;
pub use int::Int;

mod float {
    pub trait Sealed: Sized {}

    /// A floating point integer.
    pub trait Float: Sealed {}

    macro_rules! impl_float {
        ($($ident:ident),*) => {
            $(
                impl Sealed for $ident {}
                impl Float for $ident {}
            )*
        }
    }

    impl_float! { f32, f64 }
}

mod int {
    use crate::mem::Layout;

    pub trait Sealed: Sized {}

    /// An integer.
    pub trait Int: Sealed {
        #[doc(hidden)]
        fn _cake_from_be_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self;

        #[doc(hidden)]
        fn _cake_from_le_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self;

        #[doc(hidden)]
        fn _cake_from_ne_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self;
    }

    macro_rules! impl_int {
        ($($ident:ident),*) => {
            $(
                impl Sealed for $ident {}
                impl const Int for $ident {
                    fn _cake_from_be_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self
                    where
                        [(); Layout::new::<Self>().size()]:,
                    {
                        Self::from_be_bytes(bytes)
                    }

                    fn _cake_from_le_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self
                    where
                        [(); Layout::new::<Self>().size()]:,
                    {
                        Self::from_le_bytes(bytes)
                    }

                    fn _cake_from_ne_bytes(bytes: [u8; Layout::new::<Self>().size()]) -> Self
                    where
                        [(); Layout::new::<Self>().size()]:,
                    {
                        Self::from_ne_bytes(bytes)
                    }
                }
            )*
        }
    }

    impl_int! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize
    }
}

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
    [(); Layout::new::<T>().size() * 2]:,
{
    let bytes = [bytes; Layout::new::<T>().size() * 2];
    let bytes = unsafe { mem::transmute_unchecked(bytes) };

    from_ne_bytes(bytes)
}

/// Returns a `T` where every four bytes is set to `bytes`.
#[inline]
pub const fn repeat_u32<T: ~const Int>(bytes: u32) -> T
where
    Assert<{ Layout::new::<T>().size() >= 4 }>: True,
    [(); Layout::new::<T>().size()]:,
    [(); Layout::new::<T>().size() * 4]:,
{
    let bytes = [bytes; Layout::new::<T>().size() * 4];
    let bytes = unsafe { mem::transmute_unchecked(bytes) };

    from_ne_bytes(bytes)
}

/// Returns a `T` where every four bytes is set to `bytes`.
#[inline]
pub const fn repeat_u64<T: ~const Int>(bytes: u64) -> T
where
    Assert<{ Layout::new::<T>().size() >= 8 }>: True,
    [(); Layout::new::<T>().size()]:,
    [(); Layout::new::<T>().size() * 8]:,
{
    let bytes = [bytes; Layout::new::<T>().size() * 8];
    let bytes = unsafe { mem::transmute_unchecked(bytes) };

    from_ne_bytes(bytes)
}
