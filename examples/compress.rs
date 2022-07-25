use cake::compress::{Slice, Str};
use cake::mem::Layout;

fn main() {
    let xyz = Slice::new(&[1, 2, 3]);
    let hello = Str::new("hello world");

    println!("{xyz:?} {:?}", Layout::from_val(&xyz));
    println!("{hello:?} {:?}", Layout::from_val(&hello));
}
