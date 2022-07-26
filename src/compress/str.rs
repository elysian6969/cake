use super::Slice;
use core::{fmt, ops, str};

pub struct Str<'a> {
    bytes: Slice<'a, u8>,
}

impl<'a> Str<'a> {
    #[inline]
    pub fn new(string: &'a str) -> Self {
        let bytes = Slice::new(string.as_bytes());

        Self { bytes }
    }

    #[inline]
    pub fn as_str(&self) -> &'a str {
        let bytes = self.bytes.as_slice();

        unsafe { str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    pub fn as_mut_str(&mut self) -> &'a mut str {
        let bytes = self.bytes.as_mut_slice();

        unsafe { str::from_utf8_unchecked_mut(bytes) }
    }
}

impl<'a> fmt::Debug for Str<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), fmt)
    }
}

impl<'a> fmt::Display for Str<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}

impl<'a> ops::Deref for Str<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<'a> ops::DerefMut for Str<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}
