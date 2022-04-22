//! TupleArray trait and implementation.

use super::Tuple;

/// Represents a tuple containing elements of the same type.
pub trait TupleArray: Tuple {
    type Element;

    fn as_ptr(&self) -> *const Self::Element;
}

macro_rules! impl_tuple_array {
    ($($element:ident,)*) => {
        impl<T> const TupleArray for ($($element,)*) {
            type Element = T;

            fn as_ptr(&self) -> *const T {
                self as *const Self as *const T
            }
        }
    };
}

impl_tuple_array!(T,);
impl_tuple_array!(T, T,);
impl_tuple_array!(T, T, T,);
impl_tuple_array!(T, T, T, T,);
impl_tuple_array!(T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,);
impl_tuple_array!(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T,);
