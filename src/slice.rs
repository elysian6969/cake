use crate::array;
use crate::array::Array;
use crate::tuple::{FromTupleRef, Tuple};

/// Converts a reference to a tuple of `T` to an array with an equivalent length, without copying.
pub const fn from_tuple_ref<T>(tuple: &T) -> &[<T as FromTupleRef>::Element]
where
    T: ~const FromTupleRef,
    <T as FromTupleRef>::Output: ~const Array<<T as FromTupleRef>::Element>,
    [(); <T as Tuple>::LEN]:,
{
    Array::as_slice(array::from_tuple_ref(tuple))
}
