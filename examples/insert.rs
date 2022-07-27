use cake::slice;

fn main() {
    let mut foo = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let bar = [1, 2, 3];

    unsafe {
        slice::insert_slice_unchecked(&mut foo, 5, &bar);
        slice::insert_slice_unchecked(&mut foo, 3, &bar);
    }

    println!("{foo:?}");
}
