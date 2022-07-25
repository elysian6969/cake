//! Array trait and implementation.

use super::Tuple;

/// Marker trait for a contiguous tuple.
pub trait Array: Tuple {
    type Element;
}

macro_rules! impl_tuple_array {
    ($($element:ident,)*) => {
        impl<T> const Array for ($($element,)*) {
            type Element = T;
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
