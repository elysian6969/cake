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
