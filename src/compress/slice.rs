use super::WithTag;
use core::marker::PhantomData;
use core::{fmt, ops, slice};

pub struct Slice<'a, T> {
    data: WithTag<T>,
    _phantom: PhantomData<&'a [T]>,
}

impl<'a, T> Slice<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T]) -> Self {
        let address = slice.as_ptr() as *mut T;
        let len = slice.len() as u32;
        let data = WithTag::new(address, len);
        let _phantom = PhantomData;

        Self { data, _phantom }
    }

    #[inline]
    fn to_parts(&self) -> (*mut T, usize) {
        let (address, len) = self.data.to_parts();
        let len = len as usize;

        (address, len)
    }

    #[inline]
    pub fn as_slice(&self) -> &'a [T] {
        let (address, len) = self.to_parts();

        unsafe { slice::from_raw_parts(address, len) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &'a mut [T] {
        let (address, len) = self.to_parts();

        unsafe { slice::from_raw_parts_mut(address, len) }
    }
}

impl<'a, T> fmt::Debug for Slice<'a, T>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.as_slice(), fmt)
    }
}

impl<'a, T> ops::Deref for Slice<'a, T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<'a, T> ops::DerefMut for Slice<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
