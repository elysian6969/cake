use frosting::{array, slice};

fn main() {
    let tuple = ("hi", "world");
    let array = array::from_tuple_ref(&tuple);

    println!("{array:?}");

    let slice = slice::from_tuple_ref(&tuple);

    println!("{slice:?}");
}
