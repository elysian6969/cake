pub trait Sealed: Copy + Sized {}
pub trait Unsigned: Sealed {}

macro_rules! impl_unsigned {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const Unsigned for $ident {}
    )* }
}

impl_unsigned! { u8, u16, u32, u64, u128, usize }
