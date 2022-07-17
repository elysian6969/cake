use super::Signature;

#[allow(dead_code)]
#[path = "sysv/x86_64.rs"]
mod sys;

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
        sys::syscall0(id)
    }
}

impl<A> SysVSignature for fn(usize, A)
where
    A: Register,
{
    type Args = (A,);
    type Output = usize;

    unsafe fn call(self, id: usize, args: Self::Args) -> Self::Output {
        sys::syscall1(id, args.0.to_register())
    }
}

impl<A, B> SysVSignature for fn(usize, A, B)
where
    A: Register,
    B: Register,
{
    type Args = (A, B);
    type Output = usize;

    unsafe fn call(self, id: usize, args: Self::Args) -> Self::Output {
        sys::syscall2(id, args.0.to_register(), args.1.to_register())
    }
}
