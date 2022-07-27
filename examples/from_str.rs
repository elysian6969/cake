use cake::num;

const A: Option<i32> = num::from_str("5", 10);
const B: Option<i32> = num::from_str("-5", 10);

const C: Option<u32> = num::from_str("5", 10);
const D: Option<u32> = num::from_str("-5", 10);

fn main() {
    println!("{A:?}");
    println!("{B:?}");
    println!("{C:?}");
    println!("{D:?}");
}
