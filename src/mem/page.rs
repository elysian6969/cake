use core::mem::MaybeUninit;
use core::ops::Range;
use core::slice;

#[repr(C, align(4096))]
pub struct Page([MaybeUninit<u8>; 4096]);

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

// NOTE: as much as i love const, it's undefined behaviour to
// do pointer-integer casts in const eval
#[inline]
pub fn page_of<T>(ptr: *const T) -> *const Page {
    ptr.map_addr(|addr| addr & PAGE_MASK).cast()
}

#[inline]
pub fn as_page_range<T>(slice: &[T]) -> Range<*const Page> {
    let Range { start, end } = slice.as_ptr_range();
    let page_start = page_of(start).cast::<Page>();
    let page_end = unsafe { page_of(end).cast::<Page>().add(1) };

    page_start..page_end
}

#[inline]
pub unsafe fn from_page_range(range: Range<*const Page>) -> &'static [Page] {
    slice::from_ptr_range(range)
}
