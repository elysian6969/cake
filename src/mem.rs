//! Functions for dealing with memory.

use core::mem;

/// Change the lifetime of a reference.
///
/// # Safety
///
/// Caller must ensure `T` is valid for `'b`.
#[inline]
pub const unsafe fn change_lifetime<'a, 'b, T>(a: &'a T) -> &'b T {
    mem::transmute(a)
}

/// Change the lifetime of a mutable reference.
///
/// # Safety
///
/// Caller must ensure `T` is valid for `'b`.
#[inline]
pub const unsafe fn change_lifetime_mut<'a, 'b, T>(a: &'a T) -> &'b T {
    mem::transmute(a)
}
