use core::arch::asm;

#[inline(always)]
pub unsafe fn syscall0(id: usize) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}

#[inline(always)]
pub unsafe fn syscall1(id: usize, a1: usize) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        in("rdi") a1,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}

#[inline(always)]
pub unsafe fn syscall2(id: usize, a1: usize, a2: usize) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        in("rdi") a1,
        in("rsi") a2,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}

#[inline(always)]
pub unsafe fn syscall3(id: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}

#[inline(always)]
pub unsafe fn syscall4(id: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}

#[inline(always)]
pub unsafe fn syscall5(id: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}

#[inline(always)]
pub unsafe fn syscall6(
    id: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    let result: usize;

    asm!(
        "syscall",
        inout("rax") id => result,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        out("rcx") _,
        out("r11") _,
        options(nostack),
    );

    result
}
