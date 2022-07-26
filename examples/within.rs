use cake::slice;

fn main() {
    let mut items = [1, 2, 3, 4, 5];

    unsafe {
        slice::copy_within_unchecked(&mut items, 3..=4, 0);
    }

    println!("{items:?}");
}
