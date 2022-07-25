#![allow(incomplete_features)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_unsafe)]
//#![deny(warnings)]
#![feature(const_align_of_val)]
#![feature(const_align_of_val_raw)]
#![feature(const_discriminant)]
#![feature(const_maybe_uninit_array_assume_init)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_mut_refs)]
#![feature(const_option_ext)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_offset_from)]
#![feature(const_ptr_read)]
#![feature(const_refs_to_cell)]
#![feature(const_slice_from_ptr_range)]
#![feature(const_slice_index)]
#![feature(const_size_of_val)]
#![feature(const_size_of_val_raw)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(const_type_name)]
#![feature(core_intrinsics)]
#![feature(discriminant_kind)]
#![feature(generic_const_exprs)]
#![feature(layout_for_ptr)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(slice_from_ptr_range)]
#![feature(specialization)]
#![feature(type_name_of_val)]
#![feature(variant_count)]
#![no_std]

#[doc(hidden)]
pub mod macros;

pub mod array;
pub mod ffi;
pub mod marker;
pub mod mem;
pub mod slice;
pub mod traits;
pub mod tuple;
