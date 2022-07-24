use cake::mem::Layout;

fn main() {
    let x = 5_u32;
    let layout = Layout::from_val(&x);

    println!("{:?}", layout);

    #[repr(align(128))]
    struct X(u32);

    let x = X(5_u32);
    let layout = Layout::from_val(&x);

    println!("{:?}", layout);
}
