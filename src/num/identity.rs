pub trait Sealed: Copy + Sized {}

/// Multiplicative identity, `1`.
pub trait One: Sealed {
    /// Returns the multiplicative identity, `1`.
    fn one() -> Self;

    /// Returns `true` if equal to `1`.
    fn is_one(self) -> bool;
}

/// Additive identity, `0`.
pub trait Zero: Sealed {
    /// Returns the additive identity, `0`.
    fn zero() -> Self;

    /// Returns `true` if equal to `0`.
    fn is_zero(self) -> bool;
}

macro_rules! impl_identity {
    ($($ident:ident => ($zero:expr, $one:expr)),*) => { $(
        impl Sealed for $ident {}

        impl const One for $ident {
            #[inline]
            fn one() -> Self {
                $one
            }

            #[inline]
            fn is_one(self) -> bool {
                self == $one
            }
        }

        impl const Zero for $ident {
            #[inline]
            fn zero() -> Self {
                $zero
            }

            #[inline]
            fn is_zero(self) -> bool {
                self == $zero
            }
        }
    )* }
}

impl_identity! {
    i8 => (0, 1),
    i16 => (0, 1),
    i32 => (0, 1),
    i64 => (0, 1),
    i128 => (0, 1),
    isize => (0, 1),

    u8 => (0, 1),
    u16 => (0, 1),
    u32 => (0, 1),
    u64 => (0, 1),
    u128 => (0, 1),
    usize => (0, 1),

    f32 => (0.0, 1.0),
    f64 => (0.0, 1.0)
}

/// Returns the multiplicative identity, `1`.
#[inline]
pub const fn one<T>() -> T
where
    T: ~const One,
{
    One::one()
}

/// Returns the additive identity, `0`.
#[inline]
pub const fn zero<T>() -> T
where
    T: ~const Zero,
{
    Zero::zero()
}

/// Returns `true` if equal to `1`.
#[inline]
pub const fn is_one<T>(value: T) -> bool
where
    T: ~const One,
{
    value.is_one()
}

/// Returns `true` if equal to `0`.
#[inline]
pub const fn is_zero<T>(value: T) -> bool
where
    T: ~const Zero,
{
    value.is_zero()
}
