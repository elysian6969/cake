use cake::mem::MaybeUninitArray;

fn main() {
    let uninit = MaybeUninitArray::<u32, 5>::uninit();

    println!("{uninit:?}");
}
