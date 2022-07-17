use cake::ffi::sysv::SysVSignature;

fn foo(_x: usize, _y: usize) {}

fn main() {
    unsafe {
        let foo: fn(usize, usize) = foo;

        SysVSignature::call(foo, 253_usize, (1_usize,));
    }
}
