use cake::mem;

fn foo<'a, 'b: 'a>(foo: &'a str) -> &'b str {
    unsafe { mem::change_lifetime(foo) }
}

fn main() {
    let foo = foo("hi");

    println!("{foo}");
}
