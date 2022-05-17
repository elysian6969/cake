#![allow(incomplete_features)]
#![deny(warnings)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_mut_refs)]
#![feature(const_option_ext)]
#![feature(const_ptr_offset_from)]
#![feature(const_slice_from_raw_parts)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_type_name)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_uninit_array)]
#![feature(type_name_of_val)]

#[doc(hidden)]
pub mod macros;

pub mod array;
pub mod ffi;
pub mod mem;
pub mod option;
pub mod ptr;
pub mod slice;
pub mod str;
pub mod tuple;
