use cake::mem::UninitArray;

fn main() {
    let uninit = UninitArray::<u32, 5>::uninit();

    println!("{uninit:?}");
}
