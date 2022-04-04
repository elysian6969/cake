use frosting::slice;

const fn foo0() -> Option<&'static u32> {
    const SLICE: &'static [u32] = &[1, 2, 3, 4, 5];

    slice::get(SLICE, 1)
}

const fn foo1() -> Option<&'static [u32]> {
    const SLICE: &'static [u32] = &[1, 2, 3, 4, 5];

    slice::get(SLICE, 1..5)
}

fn main() {
    println!("{:?}", foo0());
    println!("{:?}", foo1());
}
