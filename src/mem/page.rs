use core::mem::MaybeUninit;
use core::ops::Range;
use core::slice;

#[repr(C, align(4096))]
pub struct Page([MaybeUninit<u8>; 4096]);

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !(PAGE_SIZE - 1);

pub fn page_of<T>(ptr: *const T) -> *const Page {
    (unsafe { core::mem::transmute::<_, usize>(ptr) } & PAGE_MASK) as *const Page
}

pub fn as_page_range<T>(slice: &[T]) -> Range<*const Page> {
    let Range { start, end } = slice.as_ptr_range();
    let page_start = page_of(start).cast::<Page>();
    let page_end = unsafe { page_of(end).cast::<Page>().add(1) };

    page_start..page_end
}

pub const unsafe fn from_page_range(range: Range<*const Page>) -> &'static [Page] {
    slice::from_ptr_range(range)
}
