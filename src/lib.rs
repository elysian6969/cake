#![allow(incomplete_features)]
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(const_mut_refs)]
#![feature(const_ptr_offset)]
#![feature(const_ptr_offset_from)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_trait_impl)]
#![feature(const_type_name)]
#![feature(generic_const_exprs)]
#![feature(inline_const)]
#![feature(type_name_of_val)]

#[doc(hidden)]
pub mod macros;

pub mod array;
pub mod slice;
pub mod tuple;
