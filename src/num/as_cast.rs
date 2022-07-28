use super::Unsigned;

pub trait Sealed: Sized {}

macro_rules! impl_sealed {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
    )* }
}

impl_sealed! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}

/// Trait representing an `as` cast.
pub trait As<T>: Sealed {
    fn cast(self) -> T;
}

/// Cast a signed integer to an unsigned integer.
pub trait AsUnsigned: Sealed {
    type Output: Unsigned;

    fn as_unsigned(self) -> Self::Output;
}

macro_rules! impl_as_unsigned {
    ($($signed:ident => $unsigned:ident),*) => { $(
        impl const AsUnsigned for $signed {
            type Output = $unsigned;

            #[inline]
            fn as_unsigned(self) -> Self::Output {
                self as  Self::Output
            }
        }
    )* }
}

impl_as_unsigned! {
    i8 => u8,
    i16 => u16,
    i32 => u32,
    i64 => u64,
    i128 => u128,
    isize => usize
}

macro_rules! impl_as_inner {
    ($($from:ident => $to:ident),*) => { $(
        impl const As<$to> for $from {
            #[inline]
            fn cast(self) -> $to {
                self as $to
            }
        }
    )* }
}

macro_rules! impl_as {
    ($($from:ident),*) => { $(
        impl_as_inner! {
            $from => i8,
            $from => i16,
            $from => i32,
            $from => i64,
            $from => i128,
            $from => isize,

            $from => u8,
            $from => u16,
            $from => u32,
            $from => u64,
            $from => u128,
            $from => usize
        }
    )* }
}

impl_as! {
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,

    i8,
    i16,
    i32,
    i64,
    i128,
    isize
}

#[inline]
pub const fn cast<T, U>(value: T) -> U
where
    T: ~const As<U>,
{
    <T as As<U>>::cast(value)
}

#[inline]
pub const fn as_unsigned<T>(value: T) -> <T as AsUnsigned>::Output
where
    T: ~const AsUnsigned,
{
    <T as AsUnsigned>::as_unsigned(value)
}
