use core::cmp::Ordering;
use core::{hint, mem};

#[derive(Copy, Debug, Eq)]
pub struct Layout {
    align: usize,
    size: usize,
}

impl Layout {
    #[inline]
    pub const fn new<T>() -> Self {
        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>();

        Self { align, size }
    }

    #[inline]
    pub const fn from_val<T: ?Sized>(value: &T) -> Self {
        let align = mem::align_of_val(value);
        let size = mem::size_of_val(value);

        Self { align, size }
    }

    #[inline]
    pub const unsafe fn from_val_raw<T: ?Sized>(value: *const T) -> Self {
        let align = mem::align_of_val_raw(value);
        let size = mem::size_of_val_raw(value);

        Self { align, size }
    }

    #[inline]
    pub const fn align(self) -> usize {
        self.align
    }

    #[inline]
    pub const fn size(self) -> usize {
        self.size
    }
}

impl const Clone for Layout {
    fn clone(&self) -> Self {
        *self
    }
}

impl const PartialEq for Layout {
    fn eq(&self, other: &Self) -> bool {
        self.align == other.align && self.size == other.size
    }

    fn ne(&self, other: &Self) -> bool {
        self.align != other.align || self.size != other.size
    }
}

impl const PartialOrd for Layout {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    fn lt(&self, other: &Self) -> bool {
        self.align < other.align || self.size < other.size
    }

    fn le(&self, other: &Self) -> bool {
        self.align <= other.align || self.size <= other.size
    }

    fn gt(&self, other: &Self) -> bool {
        self.align > other.align || self.size > other.size
    }

    fn ge(&self, other: &Self) -> bool {
        self.align >= other.align || self.size >= other.size
    }
}

impl const Ord for Layout {
    fn cmp(&self, other: &Self) -> Ordering {
        let align = cmp(self.align, other.align);
        let size = cmp(self.size, other.size);

        match (align, size) {
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Less, _) | (_, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, _) | (_, Ordering::Greater) => Ordering::Less,
        }
    }
}

const fn cmp(lhs: usize, rhs: usize) -> Ordering {
    if lhs == rhs {
        Ordering::Equal
    } else if lhs < rhs {
        Ordering::Less
    } else if lhs > rhs {
        Ordering::Greater
    } else {
        unsafe { hint::unreachable_unchecked() }
    }
}
