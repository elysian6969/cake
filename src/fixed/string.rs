use super::FixedVec;
use crate::char::{encode_utf8, next_code_point, next_code_point_reverse};
use core::{fmt, ops, str};

/// A fixed-capacity string type.
pub struct FixedString<const N: usize> {
    bytes: FixedVec<u8, N>,
}

impl<const N: usize> FixedString<N> {
    #[inline]
    pub const fn as_ptr(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    #[inline]
    pub const fn as_mut_ptr(&mut self) -> *mut u8 {
        self.bytes.as_mut_ptr()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.bytes.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    #[inline]
    pub const unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        self.bytes.as_mut_slice()
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline]
    pub const fn as_mut_str(&mut self) -> &mut str {
        unsafe { str::from_utf8_unchecked_mut(self.as_mut_bytes()) }
    }

    #[inline]
    pub const fn new() -> Self {
        let bytes = FixedVec::new();

        Self { bytes }
    }

    #[inline]
    pub const fn push(&mut self, character: char) {
        let bytes = encode_utf8(character);

        self.bytes.extend_from_slice(&bytes);
    }

    #[inline]
    pub const fn pop(&mut self) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            let character = unsafe { next_code_point_reverse(self.as_bytes())? };

            unsafe {
                self.bytes.set_len(self.len() - character.len_utf8());
            }

            Some(character)
        }
    }

    #[inline]
    pub const fn remove(&mut self, index: usize) -> Option<char> {
        if self.is_empty() {
            None
        } else {
            let prefix = &self[index..];
            let character = unsafe { next_code_point(prefix.as_bytes())? };
            let len_utf8 = character.len_utf8();

            self.bytes.copy_within((index + len_utf8).., index);

            unsafe {
                self.bytes.set_len(self.len() - len_utf8);
            }

            Some(character)
        }
    }

    #[inline]
    pub const fn insert(&mut self, index: usize, character: char) {
        let bytes = encode_utf8(character);

        unsafe {
            self.bytes.insert_slice_unchecked(index, &bytes);
        }
    }
}

impl<const N: usize> fmt::Debug for FixedString<N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), fmt)
    }
}

impl<const N: usize> fmt::Display for FixedString<N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}

impl<const N: usize> const ops::Deref for FixedString<N> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const N: usize> const ops::DerefMut for FixedString<N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}
