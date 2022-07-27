use crate::fixed::FixedVec;
use core::hint;

// based on
// next_code_point: https://raw.githubusercontent.com/rust-lang/rust/master/library/core/src/str/validations.rs
// encode_utf8: https://github.com/rust-lang/rust/blob/master/library/core/src/char/methods.rs

/// Mask of the value bits of a continuation byte.
const CONT_MASK: u8 = 0b0011_1111;

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;

/// Returns the initial codepoint accumulator for the first byte.
/// The first byte is special, only want bottom 5 bits for width 2, 4 bits
/// for width 3, and 3 bits for width 4.
#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

/// Returns the value of `ch` updated with continuation byte `byte`.
#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

/// Checks whether the byte is a UTF-8 continuation byte (i.e., starts with the
/// bits `10`).
#[inline]
const fn utf8_is_cont_byte(byte: u8) -> bool {
    (byte as i8) < -64
}

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

/// Reads the next code point out of a byte iterator (assuming a
/// UTF-8-like encoding).
///
/// # Safety
///
/// `bytes` must produce a valid UTF-8-like (UTF-8 or WTF-8) string
#[inline]
pub(crate) const unsafe fn next_code_point(bytes: &[u8]) -> Option<char> {
    // Decode UTF-8
    let x = unsafe { *bytes.get_unchecked(0) };

    if x < 128 {
        return Some(char::from_u32_unchecked(x as u32));
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]
    // NOTE: Performance is sensitive to the exact formulation here
    let init = utf8_first_byte(x, 2);

    // SAFETY: `bytes` produces an UTF-8-like string,
    // so the iterator must produce a value here.
    let y = unsafe { *bytes.get_unchecked(1) };

    let mut code = utf8_acc_cont_byte(init, y);

    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        // SAFETY: `bytes` produces an UTF-8-like string,
        // so the iterator must produce a value here.
        let z = unsafe { *bytes.get_unchecked(2) };
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);

        code = init << 12 | y_z;

        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            // SAFETY: `bytes` produces an UTF-8-like string,
            // so the iterator must produce a value here.
            let w = unsafe { *bytes.get_unchecked(3) };

            code = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    Some(char::from_u32_unchecked(code))
}

/// Reads the last code point out of a byte iterator (assuming a
/// UTF-8-like encoding).
///
/// # Safety
///
/// `bytes` must produce a valid UTF-8-like (UTF-8 or WTF-8) string
#[inline]
pub(crate) const unsafe fn next_code_point_reverse(bytes: &[u8]) -> Option<char> {
    let len = bytes.len().saturating_sub(1);

    // Decode UTF-8
    let w = unsafe { *bytes.get_unchecked(len) };

    if w < 128 {
        return Some(char::from_u32_unchecked(w as u32));
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [x [y [z w]]]

    // SAFETY: `bytes` produces an UTF-8-like string,
    // so the iterator must produce a value here.
    let z = unsafe { *bytes.get_unchecked(len.saturating_sub(1)) };
    let mut code = utf8_first_byte(z, 2);

    if utf8_is_cont_byte(z) {
        // SAFETY: `bytes` produces an UTF-8-like string,
        // so the iterator must produce a value here.
        let y = unsafe { *bytes.get_unchecked(len.saturating_sub(2)) };

        code = utf8_first_byte(y, 3);

        if utf8_is_cont_byte(y) {
            // SAFETY: `bytes` produces an UTF-8-like string,
            // so the iterator must produce a value here.
            let x = unsafe { *bytes.get_unchecked(len.saturating_sub(3)) };

            code = utf8_first_byte(x, 4);
            code = utf8_acc_cont_byte(code, y);
        }

        code = utf8_acc_cont_byte(code, z);
    }

    code = utf8_acc_cont_byte(code, w);

    Some(char::from_u32_unchecked(code))
}

pub(crate) struct Chars<'a> {
    bytes: &'a [u8],
}

impl<'a> Chars<'a> {
    #[inline]
    pub const fn new(string: &'a str) -> Self {
        let bytes = string.as_bytes();

        Self { bytes }
    }

    #[inline]
    pub const fn next(&mut self) -> Option<char> {
        if self.bytes.is_empty() {
            return None;
        }

        let character = unsafe { next_code_point(self.bytes)? };

        self.bytes = unsafe { self.bytes.get_unchecked(character.len_utf8()..) };

        Some(character)
    }
}
