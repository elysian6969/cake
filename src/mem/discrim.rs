use crate::marker::HasVariants;
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

    // handle `()` optimizations
    // e.g. Result<&str, ()> is &str
    if mem::size_of::<T>() == mem::size_of::<V>() {
        mem::transmute_copy(&value)
    } else {
        mem::transmute_copy(&Enum::<T, V> {
            discriminant,
            value,
        })
    }
}
