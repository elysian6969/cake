use super::consts::{PTR_BITS, PTR_MASK, TAG_MASK};
use core::{fmt, ptr};

/// `*mut T` but tagged.
#[repr(transparent)]
pub struct WithTag<T> {
    pointer: *mut T,
}

impl<T> WithTag<T> {
    /// Construct a new tagged pointer.
    ///
    /// `pointer` will be stripped to `pointer_BITS`.
    /// `tag` will be stripped to `TAG_BITS`.
    #[inline]
    pub fn new(pointer: *mut T, tag: u32) -> Self {
        unsafe { WithTag::from_raw(encode(pointer, tag)) }
    }

    /// Obtain a tagged pointer from an arbitary pointer.
    #[inline]
    pub unsafe fn from_raw(pointer: *mut T) -> Self {
        Self { pointer }
    }

    /// Decompose this tagged pointer into raw parts.
    #[inline]
    pub fn to_parts(self) -> (*mut T, u32) {
        decode(self.pointer)
    }

    #[inline]
    pub fn as_ptr(self) -> *mut T {
        let (pointer, _tag) = self.to_parts();

        pointer
    }

    #[inline]
    pub fn tag(self) -> u32 {
        let (_pointer, tag) = self.to_parts();

        tag
    }

    #[inline]
    pub fn with_addr(self, addr: usize) -> Self {
        let (pointer, tag) = self.to_parts();
        let pointer = pointer.with_addr(addr);

        WithTag::new(pointer, tag)
    }

    #[inline]
    pub fn with_tag(self, tag: u32) -> Self {
        let pointer = self.as_ptr();

        WithTag::new(pointer, tag)
    }
}

impl<T> Clone for WithTag<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for WithTag<T> {}

impl<T> fmt::Debug for WithTag<T> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_ptr(), fmt)
    }
}

#[inline]
fn encode<T>(pointer: *mut T, tag: u32) -> *mut T {
    let addr = pointer.expose_addr() & PTR_MASK;
    let tag = (tag as usize) & TAG_MASK;
    let shifted = tag << PTR_BITS;
    let encoded = addr | shifted;

    ptr::from_exposed_addr_mut(encoded)
}

#[inline]
fn decode<T>(pointer: *mut T) -> (*mut T, u32) {
    let addr = pointer.expose_addr();
    let pointer = ptr::from_exposed_addr_mut(addr & PTR_MASK);
    let tag = addr >> PTR_BITS;
    let tag = tag & TAG_MASK;

    (pointer, tag as u32)
}
