use cake::array;

fn modify(a: &mut i32, b: &mut i32, c: &mut i32) {
    *a = 5;
    *b = 6;
    *c = 7;
}

fn main() {
    let mut array = [1, 2, 3];
    let [x, y, z] = array::each_mut(&mut array);

    modify(x, y, z);

    println!("{array:?}");
}
