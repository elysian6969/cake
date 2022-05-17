//! Utilities related to FFI bindings.

/// Application binary interface.
pub mod abi;

/// Virtual table utilities.
pub mod vtable;

mod sealed {
    pub trait Sealed: Sized {}
}

/// A function signature.
pub trait Signature: sealed::Sealed {
    type Args;
    type Output;

    fn as_ptr(self) -> *const ();
    unsafe fn call(self, args: Self::Args) -> Self::Output;
}

/// Represents an `extern "Rust"` function signature.
///
/// Equivalent to `abi::Rust + Signature`.
pub trait RustSignature: abi::Rust + Signature {}

/// Represents an `extern "C"` function signature.
///
/// Equivalent to `abi::C + Signature`.
pub trait CSignature: abi::C + Signature {}

impl<T, Args, Output> RustSignature for T where
    T: abi::Rust + Signature<Args = Args, Output = Output>
{
}

impl<T, Args, Output> CSignature for T where T: abi::C + Signature<Args = Args, Output = Output> {}

use core::arch::asm;

pub trait SysVSignature {
    type Args;
    type Output;

    unsafe fn call(self, id: usize, args: Self::Args) -> Self::Output;
}

pub trait Register {
    unsafe fn to_register(self) -> usize;
}

macro_rules! def_register {
    ($ident:ident) => {
        impl Register for $ident {
            unsafe fn to_register(self) -> usize {
                self as usize
            }
        }
    };
}

def_register!(i64);

impl<T> Register for *const T {
    unsafe fn to_register(self) -> usize {
        self as usize
    }
}
impl<T> Register for *mut T {
    unsafe fn to_register(self) -> usize {
        self as usize
    }
}

impl<T> Register for T
where
    T: Signature,
{
    unsafe fn to_register(self) -> usize {
        self.as_ptr() as usize
    }
}

impl SysVSignature for fn(usize) {
    type Args = ();
    type Output = usize;

    unsafe fn call(self, id: usize, _args: Self::Args) -> Self::Output {
        let result: usize;

        asm!(
            "syscall",
            inlateout("rax") id => result,
            options(nostack),
        );

        result
    }
}

impl<A> SysVSignature for fn(usize, A)
where
    A: Register,
{
    type Args = (A,);
    type Output = usize;

    unsafe fn call(self, id: usize, args: Self::Args) -> Self::Output {
        let result: usize;

        asm!(
            "syscall",
            inlateout("rax") id => result,
            in("rdi") args.0.to_register(),
            options(nostack),
        );

        result
    }
}

macro_rules! impl_fn {
    ($($arg:ident,)*) => {
        // rust functions
        impl<$($arg,)* Output> sealed::Sealed for fn($($arg,)*) -> Output {}

        impl<$($arg,)* Output> Signature for fn($($arg,)*) -> Output {
            type Args = ($($arg,)*);
            type Output = Output;

            fn as_ptr(self) -> *const () {
                self as usize as *const ()
            }

            #[allow(non_snake_case)]
            #[inline]
            unsafe fn call(self, args: Self::Args) -> Self::Output {
                let ($($arg,)*) = args;

                (self)($($arg,)*)
            }
        }

        impl<$($arg,)* Output> abi::rust::Sealed for fn($($arg,)*) -> Output {}
        impl<$($arg,)* Output> abi::Rust for fn($($arg,)*) -> Output {}

        // unsafe rust functions
        impl<$($arg,)* Output> sealed::Sealed for unsafe fn($($arg,)*) -> Output {}

        impl<$($arg,)* Output> Signature for unsafe fn($($arg,)*) -> Output {
            type Args = ($($arg,)*);
            type Output = Output;

            fn as_ptr(self) -> *const () {
                self as usize as *const ()
            }

            #[allow(non_snake_case)]
            #[inline]
            unsafe fn call(self, args: Self::Args) -> Self::Output {
                let ($($arg,)*) = args;

                (self)($($arg,)*)
            }
        }

        impl<$($arg,)* Output> abi::rust::Sealed for unsafe fn($($arg,)*) -> Output {}
        impl<$($arg,)* Output> abi::Rust for unsafe fn($($arg,)*) -> Output {}

        // extern "C"-functions, these should always be marked unsafe
        impl<$($arg,)* Output> sealed::Sealed for unsafe extern "C" fn($($arg,)*) -> Output {}

        impl<$($arg,)* Output> Signature for unsafe extern "C" fn($($arg,)*) -> Output {
            type Args = ($($arg,)*);
            type Output = Output;

            fn as_ptr(self) -> *const () {
                self as usize as *const ()
            }

            #[allow(non_snake_case)]
            #[inline]
            unsafe fn call(self, args: Self::Args) -> Self::Output {
                let ($($arg,)*) = args;

                (self)($($arg,)*)
            }
        }

        impl<$($arg,)* Output> abi::c::Sealed for unsafe extern "C" fn($($arg,)*) -> Output {}
        impl<$($arg,)* Output> abi::C for unsafe extern "C" fn($($arg,)*) -> Output {}
    };
}

impl_fn!();
impl_fn!(A,);
impl_fn!(A, B,);
impl_fn!(A, B, C,);
impl_fn!(A, B, C, D,);
impl_fn!(A, B, C, D, E,);
impl_fn!(A, B, C, D, E, F,);
impl_fn!(A, B, C, D, E, F, G,);
impl_fn!(A, B, C, D, E, F, G, H,);
impl_fn!(A, B, C, D, E, F, G, H, I,);
