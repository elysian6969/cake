use cake::mem;

fn foo<'a, 'b: 'a>(foo: &'a str) -> &'b str {
    unsafe { mem::transmute_lifetime(foo) }
}

fn main() {
    let foo = foo("hi");

    println!("{foo}");
}
