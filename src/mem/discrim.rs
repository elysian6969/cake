use super::{transmute_unchecked, Layout};
use crate::marker::HasVariants;
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

/// Construct an enum variant from a `discriminantor` and `value`.
///
/// # Safety
///
/// This is incredibly unsafe!
///
/// Ensure correct ordering is applied to `value`.
///
/// ```rust
/// #[derive(Debug)]
/// enum Foo {
///     A,
///     B(&'static str),
///     C(&'static str, bool),
/// }
///
/// let c: Foo = unsafe { mem::enum_from_raw_parts(2, ("hi", true)) };
///
/// println!("{c:?}"); // segmentation fault!
/// ```
#[inline]
pub unsafe fn enum_from_raw_parts<T, V>(
    discriminant: <T as DiscriminantKind>::Discriminant,
    value: V,
) -> T
where
    T: HasVariants,
{
    let t = Layout::new::<T>();
    let v = Layout::new::<V>();

    // handle single variant optimizations
    if t == v {
        transmute_unchecked(value)
    } else {
        transmute_unchecked(WithDiscriminant::<T, V> {
            discriminant,
            value,
        })
    }
}
