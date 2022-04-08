use core::any;

pub const fn function<T: ?Sized>(function: &T) -> &'static str {
    let full_path = any::type_name_of_val(function);
    let path = &full_path[..full_path.len().saturating_sub(10)];

    path
}

#[macro_export]
macro_rules! function {
    () => {{
        fn function() {}

        $crate::macros::function(&function)
    }};
}

#[macro_export]
macro_rules! println {
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

#[macro_export]
macro_rules! offset_of {
    ($base:ident.$field:ident) => {{
        let base = core::ptr::addr_of!($base) as *const u8;
        let field = core::ptr::addr_of!($base.$field) as *const u8;

        unsafe { field.offset_from(base) }
    }};
}
