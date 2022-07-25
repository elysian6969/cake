use cake::ffi::{abi, CSignature, Signature};
use cake::tuple;

fn run_callback<C>(callback: C)
where
    C: abi::C + Signature<Args = (), Output = ()>,
{
    unsafe { callback.call(()) }
}

fn run_callback2<C>(callback: C)
where
    C: CSignature<Args = (), Output = ()>,
{
    unsafe { callback.call(()) }
}

fn run_callback3<C, T>(callback: C, tuple: T)
where
    C: CSignature<Args = T, Output = ()>,
    T: tuple::Array,
{
    unsafe { callback.call(tuple) }
}

unsafe extern "C" fn foo() {
    println!("foo");
}

unsafe extern "C" fn bar(a: u8, b: u8, c: u8) {
    println!("bar: {a} {b} {c}");
}

fn main() {
    run_callback(foo as unsafe extern "C" fn());
    run_callback2(foo as unsafe extern "C" fn());

    run_callback3(
        bar as unsafe extern "C" fn(a: u8, b: u8, c: u8),
        (1, 2, 3_u8),
    );
}
