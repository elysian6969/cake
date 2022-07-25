use cake::mem;

#[allow(dead_code)]
#[derive(Debug)]
enum Foo {
    A,
    B,
    C(&'static str),
}

fn main() {
    println!("{:?}", mem::bit_size_of::<bool>());
    println!("{:?}", mem::bit_size_of::<u8>());

    let foo = false;
    let bytes = mem::as_bytes(&foo);

    println!("{:?}", unsafe {
        mem::MaybeUninitArray::assume_init_ref(bytes)
    });

    let foo = mem::discriminant(&Foo::B);

    println!("{:?}", foo);

    let foo = mem::discriminant(&5_u32);

    println!("{:?}", foo);

    let foo = mem::variant_count::<Foo>();

    println!("{:?}", foo);

    let foo = mem::variant_count::<u32>();

    println!("{:?}", foo);

    let foo: Foo = unsafe { mem::enum_from_raw_parts(2, "hi") };

    println!("{:?}", foo);

    let foo: Result<&str, ()> = unsafe { mem::enum_from_raw_parts(0, "hi") };

    println!("{:?}", foo);

    /*enum NoVariants {}

    let foo: Result<NoVariants, _> = unsafe { mem::enum_from_raw_parts(0, ()) };

    println!("{:?}", foo);*/
}
