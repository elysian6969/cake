pub trait Sealed: Copy + Sized {}
pub trait Signed: Sealed {
    fn abs(self) -> Self;
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
    fn signum(self) -> Self;
}

macro_rules! impl_signed {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {}
        impl const Signed for $ident {
            fn abs(self) -> Self {
                self.abs()
            }

            fn is_positive(self) -> bool {
                self.is_positive()
            }

            fn is_negative(self) -> bool {
                self.is_negative()
            }

            fn signum(self) -> Self {
                self.signum()
            }
        }
    )* }
}

impl_signed! { i8, i16, i32, i64, i128, isize }
