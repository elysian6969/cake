//! Helper functions and types for arrays.

use crate::ptr;
use crate::tuple::{Tuple, TupleArray};

mod sealed {
    pub trait Sealed {}
}

/// An array!
pub trait Array<T>: sealed::Sealed {
    /// Explicitly cast to a slice.
    fn as_slice(&self) -> &[T];
}

impl<T, const N: usize> const sealed::Sealed for [T; N] {}

impl<T, const N: usize> const Array<T> for [T; N] {
    fn as_slice(&self) -> &[T] {
        self
    }
}

/// Convert a tuple of all the same type to an array, without copying.
#[inline]
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as TupleArray>::Element; <T as Tuple>::LEN]
where
    T: ~const TupleArray,
    [(); <T as Tuple>::LEN]:,
{
    unsafe { ptr::reborrow(T::as_ptr(tuple).cast()) }
}
