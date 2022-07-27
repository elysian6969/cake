use cake::slice;

fn main() {
    let mut items = [1, 2, 3, 4, 5];

    slice::copy_within(&mut items, 3..=4, 0);
    slice::copy_within(&mut items, 2, 0);

    println!("{items:?}");

    let mut items = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    slice::copy_within(&mut items, .., 5);

    println!("{items:?}");
}
