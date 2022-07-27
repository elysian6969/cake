use core::any;

/// Returns the path of a function by reference.
#[doc(hidden)]
pub const fn function_impl<T: ?Sized>(function: &T) -> &'static str {
    let full_path = any::type_name_of_val(function);
    let path = &full_path[..full_path.len().saturating_sub(10)];

    path
}

/// Calculates the distance between two pointers, *where it's known that
/// `field` is equal to or greater than `origin`*. The returned value is in
/// units of T: the distance in bytes is divided by `mem::size_of::<T>()`.
#[doc(hidden)]
pub const unsafe fn offset_of_impl<T: ?Sized, U: ?Sized>(base: *const T, field: *const U) -> usize {
    field.cast::<u8>().sub_ptr(base.cast::<u8>())
}

/// Determine the path of the current function.
#[macro_export]
macro_rules! function {
    () => {{
        fn function() {}

        $crate::macros::function_impl(&function)
    }};
}

/// `println!` prefixed with `function!`.
#[macro_export]
macro_rules! fprintln {
    () => {
        std::println!("{}", $crate::function!())
    };
    ($fmt:expr) => {
        std::println!(core::concat!("{}: ", $fmt), $crate::function!())
    };
    ($fmt:expr, $($arg:tt)*) => {
        std::println!(core::concat!("{}: ", $fmt), $crate::function!(), $($arg)*)
    };
}

/// Determine the offset of a field within a structure.
#[macro_export]
macro_rules! offset_of {
    ($base:ident.$field:ident) => {{
        let base = core::ptr::addr_of!($base);
        let field = core::ptr::addr_of!($base.$field);

        unsafe { $crate::macros::offset_of_impl(base, field) }
    }};
}
