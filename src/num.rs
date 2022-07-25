pub use float::Float;
pub use int::Int;

mod float {
    pub trait Sealed {}

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

    impl_float! {
        f32,
        f64,
    }
}

mod int {
    pub trait Sealed {}

    /// An integer.
    pub trait Int: Sealed {}

    macro_rules! impl_int {
        ($($ident:ident),*) => {
            $(
                impl Sealed for $ident {}
                impl Int for $ident {}
            )*
        }
    }

    impl_int! {
        i8, i16, i32, i64, i128, isize,
        u8, u16, u32, u64, u128, usize,
    }
}
