use crate::fixed::FixedVec;
use core::hint;

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;

#[inline]
pub const fn encode_utf8(character: char) -> FixedVec<u8, 4> {
    let len = character.len_utf8();
    let code = character as u32;
    let mut bytes = FixedVec::new();

    match len {
        1 => {
            let a = code as u8;

            bytes.push(a);
        }
        2 => {
            let a = (code >> 6 & 0x1F) as u8 | TAG_TWO_B;
            let b = (code & 0x3F) as u8 | TAG_CONT;

            bytes.push(a);
            bytes.push(b);
        }
        3 => {
            let a = (code >> 12 & 0x0F) as u8 | TAG_THREE_B;
            let b = (code >> 6 & 0x3F) as u8 | TAG_CONT;
            let c = (code & 0x3F) as u8 | TAG_CONT;

            bytes.push(a);
            bytes.push(b);
            bytes.push(c);
        }
        4 => {
            let a = (code >> 18 & 0x07) as u8 | TAG_FOUR_B;
            let b = (code >> 12 & 0x3F) as u8 | TAG_CONT;
            let c = (code >> 6 & 0x3F) as u8 | TAG_CONT;
            let d = (code & 0x3F) as u8 | TAG_CONT;

            bytes.push(a);
            bytes.push(b);
            bytes.push(c);
            bytes.push(d);
        }
        _ => unsafe { hint::unreachable_unchecked() },
    }

    bytes
}
