mod false_trait {
    use super::Assert;

    mod sealed {
        pub trait Sealed {}
    }

    pub trait False: sealed::Sealed {}

    impl sealed::Sealed for Assert<false> {}
    impl False for Assert<false> {}
}

mod true_trait {
    use super::Assert;

    mod sealed {
        pub trait Sealed {}
    }

    pub trait True: sealed::Sealed {}

    impl sealed::Sealed for Assert<true> {}
    impl True for Assert<true> {}
}

pub use false_trait::False;
pub use true_trait::True;

pub struct Assert<const VALUE: bool>;
