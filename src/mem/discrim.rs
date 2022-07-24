use crate::traits::{Assert, True};
use core::marker::DiscriminantKind;
use core::mem::ManuallyDrop;
use core::{intrinsics, mem};

/// Returns a value uniquely identifying the enum variant in `value`.
#[inline]
pub const fn discriminant<T>(value: &T) -> <T as DiscriminantKind>::Discriminant {
    intrinsics::discriminant_value(value)
}

/// Returns the number of variants in the enum type `T`.
#[inline]
pub const fn variant_count<T>() -> usize {
    mem::variant_count::<T>()
}

mod sealed {
    pub trait Sealed {}
}

pub trait HasVariants: sealed::Sealed {}

impl<T> sealed::Sealed for T where Assert<{ variant_count::<T>() > 0 }>: True {}

impl<T> HasVariants for T where T: sealed::Sealed {}

#[allow(dead_code)]
struct Enum<T, V> {
    discriminant: <T as DiscriminantKind>::Discriminant,
    value: ManuallyDrop<V>,
}

pub const unsafe fn enum_from_raw_parts<T, V>(
    discriminant: <T as DiscriminantKind>::Discriminant,
    value: V,
) -> T
where
    T: HasVariants,
{
    let value = ManuallyDrop::new(value);
    let value: T = mem::transmute_copy(&Enum::<T, V> {
        discriminant,
        value,
    });

    value
}
