#![allow(incomplete_features)]
#![deny(warnings)]
#![feature(const_mut_refs)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_type_name)]
#![feature(generic_const_exprs)]
#![feature(type_name_of_val)]

#[doc(hidden)]
pub mod macros;

pub mod array;
pub mod ffi;
pub mod mem;
pub mod ptr;
pub mod slice;
pub mod tuple;
