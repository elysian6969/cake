use cake::mem;

#[allow(dead_code)]
#[derive(Debug)]
enum Foo {
    A,
    B(&'static str),
    C(&'static str, bool),
}

fn main() {
    let b: Foo = unsafe { mem::enum_from_raw_parts(1, "this is Foo::B") };
    let result: Result<&str, ()> = unsafe { mem::enum_from_raw_parts(0, "Result::Ok") };

    println!("{:?}", b);
    println!("{:?}", result);
}
