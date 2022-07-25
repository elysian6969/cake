use cake::mem;

#[derive(Debug)]
enum NoVariants {}

fn main() {
    let example: NoVariants = unsafe { mem::enum_from_raw_parts(0, ()) };

    println!("{:?}", example);
}
