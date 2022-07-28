use cake::fixed::FixedString;
use cake::num;

const A: FixedString<12> = num::to_string::<i32, 10>(5);
const B: FixedString<12> = num::to_string::<i32, 10>(-5);
const C: FixedString<10> = num::to_string::<u32, 10>(5);
const D: FixedString<10> = num::to_string::<u32, 10>(u32::MAX);
const E: FixedString<32> = num::to_string::<u32, 2>(u32::MAX);
const F: FixedString<12> = num::to_string::<i32, 10>(i32::MIN);

fn main() {
    println!("{A:?}");
    println!("{B:?}");
    println!("{C:?}");
    println!("{D:?} ({:?}) == {:?}", D.len(), u32::MAX);
    assert_eq!(D.as_str(), format!("{:?}", u32::MAX));
    println!("{E:?} ({:?}) == {:b}", E.len(), u32::MAX);
    assert_eq!(E.as_str(), format!("{:b}", u32::MAX));
}
