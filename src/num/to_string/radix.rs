use crate::num;
use crate::num::{As, AsUnsigned, Int, One};

pub trait Sealed: Copy + Sized {}
pub trait Radix: Sealed {
    const MAX_LEN: usize;

    fn len<const RADIX: u8>() -> usize;
}

#[inline]
const fn signed_len<T, U>(radix: u8) -> usize
where
    T: ~const AsUnsigned<Output = U>,
    U: ~const Int,
    U: ~const One,
    u8: ~const As<U>,
{
    unsigned_len::<U>(radix) + 1
}

#[inline]
const fn unsigned_len<T>(radix: u8) -> usize
where
    T: ~const Int,
    T: ~const One,
    u8: ~const As<T>,
{
    let max = <T as Int>::MAX;
    let log = <T as Int>::log(max, num::cast(radix));

    log.saturating_add(1) as usize
}

macro_rules! impl_signed_radix {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const Radix for $ident {
            const MAX_LEN: usize = signed_len::<Self, _>(2);

            #[inline]
            fn len<const RADIX: u8>() -> usize {
                signed_len::<Self, _>(RADIX)
            }
        }
    )* }
}

macro_rules! impl_unsigned_radix {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
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
