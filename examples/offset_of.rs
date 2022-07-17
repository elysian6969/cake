use cake::offset_of;

#[allow(dead_code)]
#[repr(packed)]
pub struct Foo {
    foo: bool,
    bar: [u8; 6],
    foobar: bool,
}

fn main() {
    let foo = Foo {
        foo: true,
        bar: [1, 2, 3, 4, 5, 6],
        foobar: true,
    };

    println!("{:?}", offset_of!(foo.foo));
    println!("{:?}", offset_of!(foo.foobar));
}
