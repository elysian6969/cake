#![allow(incomplete_features)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_type_name)]
#![feature(generic_const_exprs)]
#![feature(type_name_of_val)]

#[doc(hidden)]
pub mod macros;

#[cfg(feature = "tuple")]
pub mod array;

#[cfg(feature = "tuple")]
pub mod slice;

#[cfg(feature = "tuple")]
pub mod tuple;
