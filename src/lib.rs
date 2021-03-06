#![allow(incomplete_features)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(unused_unsafe)]
#![deny(warnings)]
// generic
#![feature(const_mut_refs)]
#![feature(const_option_ext)]
#![feature(const_precise_live_drops)]
#![feature(const_ptr_read)]
#![feature(const_refs_to_cell)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(const_slice_index)]
#![feature(const_trait_impl)]
#![feature(const_transmute_copy)]
#![feature(generic_const_exprs)]
#![feature(slice_from_ptr_range)]
// src/fixed
#![feature(const_maybe_uninit_write)]
// src/fixed/string
#![feature(const_char_convert)]
#![feature(const_convert)]
#![feature(const_str_from_utf8_unchecked_mut)]
#![feature(const_try)]
// src/fixed/vec
#![feature(const_ptr_write)]
// src/compress
#![feature(const_pointer_byte_offsets)]
#![feature(pointer_byte_offsets)]
// src/macros
#![feature(const_ptr_sub_ptr)]
#![feature(const_type_name)]
#![feature(ptr_sub_ptr)]
#![feature(type_name_of_val)]
// src/mem/discrim
#![feature(const_discriminant)]
#![feature(core_intrinsics)]
#![feature(discriminant_kind)]
#![feature(variant_count)]
// src/mem/layout
#![feature(const_align_of_val)]
#![feature(const_align_of_val_raw)]
#![feature(const_cmp)]
#![feature(const_size_of_val)]
#![feature(const_size_of_val_raw)]
#![feature(layout_for_ptr)]
// src/mem/page
#![feature(strict_provenance)]
// src/mem/uninit
#![feature(const_maybe_uninit_array_assume_init)]
#![feature(const_maybe_uninit_uninit_array)]
#![feature(const_replace)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
// src/num
#![feature(const_fn_floating_point_arithmetic)]
#![feature(int_log)]
#![no_std]

#[doc(hidden)]
pub mod macros;

pub mod array;
pub mod char;
#[cfg(target_pointer_width = "64")]
pub mod compress;
pub mod ffi;
pub mod fixed;
pub mod marker;
pub mod mem;
pub mod num;
pub mod slice;
pub mod traits;
pub mod tuple;
