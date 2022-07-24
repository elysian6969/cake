//! Functions for dealing with memory.

use crate::traits::{Assert, True};
use core::mem::ManuallyDrop;
use core::{mem, ptr};

pub use discrim::{discriminant, enum_from_raw_parts, variant_count, HasVariants};

pub use layout::Layout;
pub use uninit::MaybeUninitArray;

mod discrim;
mod layout;
mod uninit;

/// Determines whether `T` could be transmuted to `U`.
#[inline]
pub const fn is_transmutable<T, U>() -> bool {
    let t = Layout::new::<T>();
    let u = Layout::new::<U>();

    t == u
}

/// Determines whether `T` could be transmuted to `U`.
#[inline]
pub const fn is_transmutable_val<T: ?Sized, U: ?Sized>(t: &T, u: &U) -> bool {
    let t = Layout::from_val(t);
    let u = Layout::from_val(u);

    t == u
}

/// Determines whether `T` could be transmuted to `U`.
#[inline]
pub const unsafe fn is_transmutable_val_raw<T: ?Sized, U: ?Sized>(
    t: *const T,
    u: *const U,
) -> bool {
    let t = Layout::from_val_raw(t);
    let u = Layout::from_val_raw(u);

    t == u
}

/// Reinterprets the bits of a value of one type as another type.
///
/// Semantically equivalent to a bitwise move of one type into another.
/// It copies the bits from `value` into the destination, then forgets
/// `value`.
///
/// # Safety
///
/// Both types must have the same size. Neither the original, nor
/// the result, may be an invalid value.
#[inline]
pub const unsafe fn transmute_unchecked<T, U>(value: T) -> U {
    let value = ManuallyDrop::new(value);

    mem::transmute_copy(&value)
}

#[inline]
pub const unsafe fn transmute<T, U>(value: T) -> U
where
    Assert<{ is_transmutable::<T, U>() }>: True,
{
    transmute_unchecked(value)
}

/// Reinterprets the bits of an array as another array.
///
/// Everything from [`core::mem::transmute`](core::mem::transmute) applies.
#[inline]
pub const unsafe fn transmute_array<T, U, const N: usize>(array: [T; N]) -> [U; N] {
    transmute_unchecked(array)
}

/// Interprets `src` as having type `&U`, and then reads `src` without
/// moving the contained value.
///
/// Differences to [`core::mem::transmute_copy`](core::mem::transmute_copy)
///
/// - No `Sized` constraint on `T`.
/// - Ensures [undefined behaviour](https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html)
///   isn't trigged by `U` being larger than `T`.
///
#[inline]
pub const unsafe fn transmute_copy<T, U>(src: &T) -> Option<U>
where
    T: ?Sized,
{
    let t = Layout::from_val_raw(src);
    let u = Layout::new::<U>();

    if u.size() > t.size() {
        return None;
    }

    let value = if u.align() > t.align() {
        unsafe { ptr::read_unaligned(src as *const T as *const U) }
    } else {
        unsafe { ptr::read(src as *const T as *const U) }
    };

    Some(value)
}

/// Transmute lifetime `'a` to lifetime `'b`.
#[inline]
pub const unsafe fn change_lifetime<'a, 'b, T>(value: &'a T) -> &'b T
where
    T: ?Sized,
{
    &*(value as *const T)
}

/// Transmute lifetime `'a` to lifetime `'b`.
#[inline]
pub const unsafe fn change_lifetime_mut<'a, 'b, T>(value: &'a mut T) -> &'b mut T
where
    T: ?Sized,
{
    &mut *(value as *mut T)
}

pub const POINTER_SIZE: usize = mem::size_of::<bool>();

// ty https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-utils/src/cache_padded.rs
#[cfg(target_arch = "s390x")]
pub const CACHE_LINE_ALIGN: usize = 256;

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "powerpc64",
    target_arch = "x86_64",
))]
pub const CACHE_LINE_ALIGN: usize = 128;

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc64",
    target_arch = "riscv64",
    target_arch = "s390x",
    target_arch = "x86_64",
)))]
pub const CACHE_LINE_ALIGN: usize = 64;

#[cfg(any(
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "riscv64",
))]
pub const CACHE_LINE_ALIGN: usize = 32;

#[inline]
pub const fn is_zero_sized<T>() -> bool {
    mem::size_of::<T>() == 0
}

#[inline]
pub const fn is_pointer_sized<T>() -> bool {
    mem::size_of::<T>() == POINTER_SIZE
}

#[inline]
pub const fn is_cache_line_aligned<T>() -> bool {
    mem::align_of::<T>() == CACHE_LINE_ALIGN
}

#[inline]
pub const fn bit_size_of<T>() -> usize {
    mem::size_of::<T>().saturating_mul(8)
}

#[inline]
pub const fn as_bytes<T>(value: &T) -> &MaybeUninitArray<u8, { mem::size_of::<T>() }> {
    unsafe { &*(value as *const T as *const MaybeUninitArray<u8, { mem::size_of::<T>() }>) }
}
