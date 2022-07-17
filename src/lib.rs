#![allow(incomplete_features)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_unsafe)]
#![deny(warnings)]
#![feature(const_align_of_val)]
#![feature(const_maybe_uninit_array_assume_init)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_mut_refs)]
#![feature(const_option_ext)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_offset_from)]
#![feature(const_ptr_read)]
#![feature(const_refs_to_cell)]
#![feature(const_size_of_val)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(const_type_name)]
#![feature(generic_const_exprs)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(type_name_of_val)]
#![no_std]

#[doc(hidden)]
pub mod macros;

pub mod array;
pub mod ffi;
pub mod mem;
pub mod slice;
pub mod tuple;
