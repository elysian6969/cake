mod sealed {
    pub trait Sealed {}
}

/// Represents a tuple.
pub trait Tuple: sealed::Sealed {
    const LEN: usize;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

/// Trait behind the magic of `array::from_tuple_ref`.
pub trait FromTupleRef: Tuple {
    type Element;
    type Output;

    fn from_tuple_ref(&self) -> &Self::Output;
}

macro_rules! impl_tuple {
    ($($element:ident),*; ?$unsized:ident; $len:literal) => {
        impl<$($element),*> const sealed::Sealed for ($($element),*,)
            where $unsized: ?Sized
        {}

        impl<$($element),*> const Tuple for ($($element),*,)
            where $unsized: ?Sized
        {
            const LEN: usize = $len;

            fn is_empty(&self) -> bool {
                false
            }

            fn len(&self) -> usize {
                $len
            }
        }
    };
}

macro_rules! impl_from_tuple_ref {
    ($($element:ident),*) => {
        impl<T> const FromTupleRef for ($($element),*,)
            where [(); <Self as Tuple>::LEN]:,
        {
            type Element = T;
            type Output = [T; <Self as Tuple>::LEN];

            fn from_tuple_ref(&self) -> &Self::Output {
                unsafe {
                    &*(self as *const Self as *const Self::Output)
                }
            }
        }
    };
}

impl_tuple!(A; ?A; 1);
impl_tuple!(A, B; ?B; 2);
impl_tuple!(A, B, C; ?C; 3);
impl_tuple!(A, B, C, D; ?D; 4);
impl_tuple!(A, B, C, D, E; ?E; 5);
impl_tuple!(A, B, C, D, E, F; ?F; 6);
impl_tuple!(A, B, C, D, E, F, G; ?G; 7);
impl_tuple!(A, B, C, D, E, F, G, H; ?H; 8);
impl_tuple!(A, B, C, D, E, F, G, H, I; ?I; 9);
impl_tuple!(A, B, C, D, E, F, G, H, I, J; ?J; 10);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K; ?K; 11);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L; ?L; 12);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M; ?M; 13);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N; ?N; 14);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O; ?O; 15);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P; ?P; 16);

impl_from_tuple_ref!(T);
impl_from_tuple_ref!(T, T);
impl_from_tuple_ref!(T, T, T);
impl_from_tuple_ref!(T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T);
impl_from_tuple_ref!(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T);
