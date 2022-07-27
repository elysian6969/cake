#![feature(const_mut_refs)]

use cake::fixed::{FixedString, FixedVec};

const fn yeah() -> (FixedVec<i32, 3>, FixedVec<i32, 30>, FixedString<30>) {
    let mut vec = FixedVec::<_, 3>::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut vec2 = FixedVec::<_, 30>::new();

    vec2.extend_from_slice(&[4, 5, 6]);
    vec2.insert_from_slice(0, &[1, 2, 3]);
    vec2.insert_from_slice(0, &[1, 2, 3]);

    let mut string = FixedString::<30>::new();

    string.insert(0, 'o');
    string.insert(0, 'l');
    string.insert(0, 'l');
    string.insert(0, 'e');
    string.insert(0, 'h');

    (vec, vec2, string)
}

const YEET: (FixedVec<i32, 3>, FixedVec<i32, 30>, FixedString<30>) = yeah();

fn main() {
    println!("{YEET:?}");

    let mut vec = FixedVec::<_, 3>::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{vec:?}");
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{:?}", vec.pop());
    println!("{vec:?}");
    
    let mut vec = FixedVec::<_, 30>::new();

    vec.extend_from_slice(&[4, 5, 6]);
    vec.insert_from_slice(0, &[1, 2, 3]);
    vec.insert_from_slice(0, &[1, 2, 3]);
    
    println!("{vec:?}");

    let mut string = FixedString::<30>::new();

    string.push('h');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');

    println!("string = {string}");
    println!("string = {string:?}");

    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());
    println!("{:?}", string.pop());

    println!("string = {string}");
    println!("string = {string:?}");

    string.push('h');
    string.push('e');
    string.push('l');
    string.push('l');
    string.push('o');

    println!("string = {string}");
    println!("string = {string:?}");

    println!("{:?}", string.remove(0));
    println!("{:?}", string.remove(0));
    println!("{:?}", string.remove(0));
    println!("{:?}", string.remove(0));
    println!("{:?}", string.remove(0));
    println!("{:?}", string.remove(0));

    println!("string = {string}");
    println!("string = {string:?}");

    string.insert(0, 'o');
    string.insert(0, 'l');
    string.insert(0, 'l');
    string.insert(0, 'e');
    string.insert(0, 'h');

    println!("string = {string}");
    println!("string = {string:?}");
}
