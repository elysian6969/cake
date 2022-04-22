pub(crate) mod c {
    mod sealed {
        pub trait Sealed {}
    }

    pub(crate) use sealed::Sealed;

    /// Trait representing `extern "C"`, and `#[repr(C)]`.
    pub trait C: Sealed {}
}

pub(crate) mod rust {
    mod sealed {
        pub trait Sealed {}
    }

    pub(crate) use sealed::Sealed;

    /// Trait representing `extern "Rust"`, and `#[repr(Rust)]`.
    pub trait Rust: Sealed {}
}

pub use c::C;
pub use rust::Rust;
