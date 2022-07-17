//! Functions for dealing with memory.

use core::{mem, ptr};

pub use uninit::MaybeUninitArray;

mod uninit;

pub const unsafe fn transmute_array<T, U, const N: usize>(array: [T; N]) -> [U; N] {
    let new_array = mem::transmute_copy(&array);

    mem::forget(array);

    new_array
}

/// Interprets `src` as having type `&U`, and then reads `src` without
/// moving the contained value.
///
/// Differences to `core::mem::transmute_copy`:
///
/// - Accepts an unsized `T`.
/// - Will return `None` if `U` is larger than `T`.
///
pub const unsafe fn transmute_copy<T, U>(src: &T) -> Option<U>
where
    T: ?Sized,
{
    let (align_t, size_t) = (mem::align_of_val(src), mem::size_of_val(src));
    let (align_u, size_u) = (mem::align_of::<U>(), mem::size_of::<U>());

    if size_u > size_t {
        return None;
    }

    let value = if align_u > align_t {
        // SAFETY: n
        unsafe { ptr::read_unaligned(src as *const T as *const U) }
    } else {
        // SAFETY: n
        unsafe { ptr::read(src as *const T as *const U) }
    };

    Some(value)
}

/// Transmute lifetime `'a` to lifetime `'b`.
pub const unsafe fn change_lifetime<'a, 'b, T>(value: &'a T) -> &'b T
where
    T: ?Sized,
{
    &*(value as *const T)
}

/// Transmute lifetime `'a` to lifetime `'b`.
pub const unsafe fn change_lifetime_mut<'a, 'b, T>(value: &'a mut T) -> &'b mut T
where
    T: ?Sized,
{
    &mut *(value as *mut T)
}
