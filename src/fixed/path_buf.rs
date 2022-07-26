use super::FixedVec;
use crate::char::{encode_utf8, next_code_point, next_code_point_reverse};
use crate::slice;
use core::{fmt, ops, str};

/// A fixed-capacity string type.
pub struct FixedPathBuf<const N: usize> {
    bytes: FixedVec<u8, N>,
}

impl<const N: usize> FixedPathBuf<N> {
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
    pub const fn new() -> Self {
        let bytes = FixedVec::new();

        Self { bytes }
    }
}

impl<const N: usize> fmt::Debug for FixedPathBuf<N> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_bytes(), fmt)
    }
}
