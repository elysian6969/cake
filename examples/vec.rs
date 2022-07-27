#![feature(const_mut_refs)]

use cake::fixed::{FixedString, FixedVec};

const fn yeah() -> (FixedVec<i32, 30>, FixedString<30>) {
    let mut vec = FixedVec::<_, 30>::new();

    vec.extend_from_slice(&[4, 5, 6]);

    unsafe {
        vec.insert_slice_unchecked(0, &[1, 2, 3]);
        vec.insert_slice_unchecked(0, &[1, 2, 3]);
    }

    let mut string = FixedString::<30>::new();

    string.insert(0, 'o');
    string.insert(0, 'l');
    string.insert(0, 'l');
    string.insert(0, 'e');
    string.insert(0, 'h');

    (vec, string)
}

const YEET: (FixedVec<i32, 30>, FixedString<30>) = yeah();

fn main() {
    println!("YEET = {YEET:?}");
}
