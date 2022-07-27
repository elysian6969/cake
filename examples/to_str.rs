use cake::fixed::FixedString;
use cake::num::to_string::signed_to_string;

/*const A: Option<&str> = num::to_str(5_i32, 10);
const B: Option<&str> = num::to_str(-5_i32, 10);

const C: Option<&str> = num::to_str(5_u32, 10);*/
//const D: Option<&str> = num::to_str(-5_u32, 10);

fn main() {
    /*println!("{A:?}");
    println!("{B:?}");
    println!("{C:?}");*/
    //println!("{D:?}");

    const RESULT: FixedString<32> = signed_to_string::<i32, 2_u8>(69420);

    println!("{RESULT:?}");
    println!("\"{:0b}\"", 69420_i32);
}
