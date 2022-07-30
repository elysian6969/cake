use cake::ffi::sysv;
use cake::ffi::sysv::Arg;

unsafe fn foo<A: Arg>(a: A) {}

fn main() {
    unsafe {
        foo(5);

        sysv::call(0, (1_i32,));
        sysv::call(0, (1_u32,));
    }
}
