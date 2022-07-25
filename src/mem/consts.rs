use super::Layout;

pub const POINTER_SIZE: usize = Layout::new::<usize>().size();

// ty https://github.com/crossbeam-rs/crossbeam/blob/master/crossbeam-utils/src/cache_padded.rs
#[cfg(target_arch = "s390x")]
pub const CACHE_LINE_SIZE: usize = 256;

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "powerpc64",
    target_arch = "x86_64",
))]
pub const CACHE_LINE_SIZE: usize = 128;

#[cfg(not(any(
    target_arch = "aarch64",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc64",
    target_arch = "riscv64",
    target_arch = "s390x",
    target_arch = "x86_64",
)))]
pub const CACHE_LINE_SIZE: usize = 64;

#[cfg(any(
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "riscv64",
))]
pub const CACHE_LINE_SIZE: usize = 32;
