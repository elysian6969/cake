use cake::array;

fn main() {
    let mut array = [1, 2, 3];

    {
        let [x, y, z] = array::each_mut(&mut array);

        *x = 5;
        *y = 6;
        *z = 7;
    }

    println!("{array:?}");
}
