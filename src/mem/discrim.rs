use crate::marker::HasVariants;
use crate::mem::transmute_unchecked;
use core::marker::DiscriminantKind;
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
struct WithDiscriminant<T, V> {
    discriminant: <T as DiscriminantKind>::Discriminant,
    value: V,
}

pub const unsafe fn enum_from_raw_parts<T, V>(
    discriminant: <T as DiscriminantKind>::Discriminant,
    value: V,
) -> T
where
    T: HasVariants,
{
    // handle `()` optimizations
    // e.g. Result<&str, ()> is &str
    if mem::size_of::<T>() == mem::size_of::<V>() {
        transmute_unchecked(value)
    } else {
        transmute_unchecked(WithDiscriminant::<T, V> {
            discriminant,
            value,
        })
    }
}
