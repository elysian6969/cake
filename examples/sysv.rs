use cake::ffi::sysv;
use cake::ffi::sysv::{Call, Arg};

use core::arch::asm;

unsafe fn foo<A: Arg>(a: A) {}

cake::impl_call! {
    (A,);
    "syscall";
    inout("rax") id => result,
    out("r11") _,
    out("rcx") _,
}

fn main() {
    unsafe { foo(5); }
}
