use cake::mem;
use cake::mem::Layout;

#[allow(dead_code)]
#[derive(Debug)]
enum Foo {
    A,
    B(&'static str),
    C(&'static str, bool),
}

fn main() {
    println!("{:?}", Layout::new::<Foo>());

    let b: Foo = unsafe { mem::enum_from_raw_parts(1, "hi") };
    let c: Foo = unsafe { mem::enum_from_raw_parts(2, ("hi", true)) };
    let result: Result<&str, ()> = unsafe { mem::enum_from_raw_parts(0, "hi") };

    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", result);
}
