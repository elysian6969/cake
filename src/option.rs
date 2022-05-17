use super::slice;
use core::ptr;

const fn null<T>() -> (*const T, usize) {
    (ptr::null(), 0)
}

/// Convert an `Option<&[T]>` into a pointer and length.
pub const fn to_raw_parts<'a, I, T>(slice: I) -> (*const T, usize)
where
    I: ~const Into<Option<&'a [T]>>,
    T: 'a,
{
    let slice = slice.into();
    let (address, len) = slice.map(slice::to_raw_parts).unwrap_or_else(null);

    (address, len)
}
