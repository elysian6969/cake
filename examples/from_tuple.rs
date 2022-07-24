#![feature(type_name_of_val)]

use cake::{array, slice};
use core::any;

fn main() {
    let tuple = ("hi", "world");
    let array = array::from_tuple_ref(&tuple);

    println!("array: {} = {array:?}", any::type_name_of_val(&array));

    let slice = slice::from_tuple_ref(&tuple);

    println!("slice: {} = {slice:?}", any::type_name_of_val(&slice));

    let array = array::from_tuple(tuple);

    println!("array: {} = {array:?}", any::type_name_of_val(&array));
}
