use cake::compress::Slice;

fn main() {
    let example = Slice::new(&[1, 2, 3]);

    println!("{example:?}");
}
