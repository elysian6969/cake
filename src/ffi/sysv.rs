//use super::Signature;

/*#[allow(dead_code)]
#[path = "sysv/x86_64.rs"]
mod sys;*/

/*pub trait SysVSignature {
    type Args;
    type Output;

    unsafe fn call(self, id: usize, args: Self::Args) -> Self::Output;
}*/

pub trait Sealed: Copy + Sized {
    type Output;

    fn into(self) -> Self::Output;
}

/// A trait representing types passable to asm registers.
pub trait Arg: Sealed {}

macro_rules! impl_args {
    ($($ident:ident),*) => { $(
        impl Sealed for $ident {
            type Output = $ident;

            fn into(self) -> Self::Output {
                self
            }
        }

        impl Arg for $ident {}
    )* }
}

impl_args! { bool, f32, f64, i8, i16, isize, u8, u16, usize }

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_args! { i32, u32 }

#[cfg(target_pointer_width = "64")]
impl_args! { i64, u64 }

impl<T: ?Sized> Sealed for *const T {
    type Output = *const T;

    fn into(self) -> Self::Output {
        self
    }
}

impl<T: ?Sized> Sealed for *mut T {
    type Output = *mut T;

    fn into(self) -> Self::Output {
        self
    }
}

impl<T: ?Sized> Arg for *const T {}
impl<T: ?Sized> Arg for *mut T {}

use core::arch::asm;

pub trait Call<Args> {
    type Output;

    unsafe fn exec(self, args: Args) -> Self::Output;
}

macro_rules! impl_call {
    (
        $syscall:literal;
        ($($element:ident,)*);
        ($id:ident, $args:ident, $result:ident);
        $($asm:tt)*
    ) => {
        impl Call<($($element,)*)> for usize
        where
            $($element: Arg,)*
        {
            type Output = usize;

            #[inline(always)]
            unsafe fn exec(self, $args: ($($element,)*)) -> Self::Output {
                let $id = self;
                let $result: usize;
                
                asm!(
                    $syscall,
                    $($asm)*
                    options(nostack),
                );

                $result
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
macro_rules! impl_calls {
    ($($ident:ident,)*) => { $(
        impl_call! {
            "svc 0";
            ($ident,);
            (id, args, result);
            in("x8") id,
            inout("x0") args.0 as usize => result,
        }
    )* };
}

impl_calls! { i16, i32, u16, u32, }

/*#[cfg(target_arch = "x86_64")]
impl_call! {
    "syscall";
    (A,);
    inout("rax") id => result,
    out("r11") _,
    out("rcx") _,
}*/

pub unsafe fn call<Args, Output>(id: usize, args: Args) -> <usize as Call<Args>>::Output
where
    usize: Call<Args, Output = Output>,
{
    <usize as Call<Args>>::exec(id, args)
}

/*macro_rules! impl_calls {
    (internal $syscall:literal;) => {
        //
    };

    // consume inout("reg") args.0 => result
    (internal $syscall:literal; $dir:ident($reg:literal) $op0:ident.$op1:expr => result, $($tail:tt)*) => {
        impl_call! { $syscall; $($tail)* }
        impl_calls! { internal $syscall; $($tail)* }
    };

    // consume inout("reg") id => result
    (internal $syscall:literal; $dir:ident($reg:literal) $op:ident => result, $($tail:tt)*) => {
        impl_call! { $syscall; $($tail)* }
        impl_calls! { internal $syscall; $($tail)* }
    };

    // consume in("reg") args.0
    // consume out("reg") args.0
    (internal $syscall:literal; $dir:ident($reg:literal) $op0:ident.$op1:expr, $($tail:tt)*) => {
        impl_call! { $syscall; $($tail)* }
        impl_calls! { internal $syscall; $($tail)* }
    };

    // consume in("reg") _
    // consume out("reg") _
    (internal $syscall:literal; $dir:ident($reg:literal) _, $($tail:tt)*) => {
        impl_call! { $syscall; $($tail)* }
        impl_calls! { internal $syscall; $($tail)* }
    };

    ($syscall:literal; clobbers($($clobber:literal),*); $($tail:tt)*) => {
        impl_calls! { internal $syscall; $($tail)* $(out($clobber) _,)*  }
    }
}

#[cfg(target_arch = "aarch64")]
impl_calls! {
    "svc 0";
    clobbers();
    in("x8") id;
    inout("x0") args.0 => result;
    in("x1") args.1;
    in("x2") args.2;
    in("x3") args.3;
    in("x4") args.4;
    in("x5") args.5;
}

#[cfg(target_arch = "x86_64")]
impl_calls! {
    "syscall";
    clobbers("rcx", "r11");
    in("r9") args.5,
    in("r8") args.4,
    in("r10") args.3,
    in("rdx") args.2,
    in("rsi") args.1,
    in("rdi") args.0,
    inout("rax") id => result,
}

#[cfg(target_arch = "riscv64")]
impl_calls! {
    "ecall";
    clobbers();
    in("x17") id,
    inout("x10") args.0 => result;
    in("x11") args.1;
    in("x12") args.2;
    in("x13") args.3;
    in("x14") args.4;
    in("x15") args.5;
}*/
