pub use has_variants::HasVariants;

mod has_variants {
    use crate::mem;
    use crate::traits::{Assert, True};

    pub trait Sealed {}

    /// A marker trait for enums that contain variants.
    pub trait HasVariants: Sealed {}

    #[inline]
    const fn has_variants<T>() -> bool {
        mem::variant_count::<T>() > 0
    }

    impl<T> Sealed for T where Assert<{ has_variants::<T>() }>: True {}
    impl<T> HasVariants for T where T: Sealed {}
}
