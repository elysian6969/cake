//! Functions for dealing with memory.

use crate::traits::{Assert, True};
use core::mem::ManuallyDrop;
use core::ptr;

pub use consts::{CACHE_LINE_SIZE, POINTER_SIZE};
pub use discrim::{discriminant, enum_from_raw_parts, variant_count};
pub use layout::Layout;
pub use uninit::UninitArray;

mod consts;
mod discrim;
mod layout;
mod uninit;

pub mod page;

#[inline]
pub const fn is_zero_sized<T>() -> bool {
    Layout::new::<T>().size() == 0
}

#[inline]
pub const fn is_pointer_sized<T>() -> bool {
    Layout::new::<T>().size() == POINTER_SIZE
}

#[inline]
pub const fn is_cache_line_aligned<T>() -> bool {
    Layout::new::<T>().align() == CACHE_LINE_SIZE
}

#[inline]
pub const fn bit_size_of<T>() -> usize {
    Layout::new::<T>().size().saturating_mul(8)
}

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
/// Neither the original, nor the result, may be an invalid value.
#[inline]
pub const unsafe fn transmute<T, U>(value: T) -> U
where
    Assert<{ is_transmutable::<T, U>() }>: True,
{
    transmute_unchecked(value)
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

    transmute_copy_unchecked(&value)
}

/// Reinterprets the bits of an array as another array.
///
/// Everything from [`core::mem::transmute`](core::mem::transmute) applies.
#[inline]
pub const unsafe fn transmute_array<T, U, const N: usize>(array: [T; N]) -> [U; N]
where
    Assert<{ is_transmutable::<T, U>() }>: True,
{
    transmute_array_unchecked(array)
}

/// Reinterprets the bits of an array as another array.
///
/// Everything from [`core::mem::transmute`](core::mem::transmute) applies.
#[inline]
pub const unsafe fn transmute_array_unchecked<T, U, const N: usize>(array: [T; N]) -> [U; N] {
    transmute_unchecked(array)
}

/// Interprets `value` as having type `&U`, and then reads `src` without
/// moving the contained value.
///
/// Differences to [`core::mem::transmute_copy`](core::mem::transmute_copy)
///
/// - No `Sized` constraint on `T`.
/// - Ensures [undefined behaviour](https://doc.rust-lang.org/nightly/reference/behavior-considered-undefined.html)
///   isn't trigged by `U` being larger than `T`.
///
#[inline]
pub const unsafe fn transmute_copy<T, U>(value: &T) -> Option<U>
where
    T: ?Sized,
{
    let t = Layout::from_val_raw(value);
    let u = Layout::new::<U>();

    if u.size() > t.size() {
        return None;
    }

    Some(transmute_copy_unchecked(value))
}

/// Interprets `value` as having type `&U`, and then reads `src` without
/// moving the contained value.
///
/// Differences to [`core::mem::transmute_copy`](core::mem::transmute_copy)
///
/// - No `Sized` constraint on `T`.
///
#[inline]
pub const unsafe fn transmute_copy_unchecked<T, U>(value: &T) -> U
where
    T: ?Sized,
{
    let t = Layout::from_val_raw(value);
    let u = Layout::new::<U>();
    let value = value as *const T as *const U;

    // copy optimization
    let value = if u.align() > t.align() {
        unsafe { ptr::read_unaligned(value) }
    } else {
        unsafe { ptr::read(value) }
    };

    value
}

/// Transmute lifetime `'a` to lifetime `'b`.
#[inline]
pub const unsafe fn transmute_lifetime<'a, 'b, T>(value: &'a T) -> &'b T
where
    T: ?Sized,
{
    &*(value as *const T)
}

/// Transmute lifetime `'a` to lifetime `'b`.
#[inline]
pub const unsafe fn transmute_lifetime_mut<'a, 'b, T>(value: &'a mut T) -> &'b mut T
where
    T: ?Sized,
{
    &mut *(value as *mut T)
}
