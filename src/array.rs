use crate::tuple::{FromTupleRef, Tuple};

mod sealed {
    pub trait Sealed {}
}

pub trait Array<T>: sealed::Sealed {
    fn as_slice(&self) -> &[T];
}

impl<T, const N: usize> const sealed::Sealed for [T; N] {}

impl<T, const N: usize> const Array<T> for [T; N] {
    fn as_slice(&self) -> &[T] {
        self
    }
}

/// Converts a reference to a tuple of `T` to an array with an equivalent length, without copying.
pub const fn from_tuple_ref<T>(tuple: &T) -> &<T as FromTupleRef>::Output
where
    T: ~const FromTupleRef,
    <T as FromTupleRef>::Output: ~const Array<<T as FromTupleRef>::Element>,
    [(); <T as Tuple>::LEN]:,
{
    <T as FromTupleRef>::from_tuple_ref(tuple)
}
