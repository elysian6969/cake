//! Helper functions and types for tuples.

mod array;
mod index;

pub use array::TupleArray;
pub use index::TupleIndex;

mod sealed {
    pub trait Sealed {}
}

/// Represents a tuple.
pub trait Tuple: sealed::Sealed {
    const LEN: usize;

    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
}

macro_rules! impl_tuple {
    ($($element:ident,)*; ?$unsized:ident; $len:literal) => {
        impl<$($element,)*> const sealed::Sealed for ($($element,)*)
            where $unsized: ?Sized
        {}

        impl<$($element,)*> const Tuple for ($($element,)*)
            where $unsized: ?Sized
        {
            const LEN: usize = $len;

            fn is_empty(&self) -> bool {
                $len == 0
            }

            fn len(&self) -> usize {
                $len
            }
        }
    };
}

impl_tuple!(A,; ?A; 1);
impl_tuple!(A, B,; ?B; 2);
impl_tuple!(A, B, C,; ?C; 3);
impl_tuple!(A, B, C, D,; ?D; 4);
impl_tuple!(A, B, C, D, E,; ?E; 5);
impl_tuple!(A, B, C, D, E, F,; ?F; 6);
impl_tuple!(A, B, C, D, E, F, G,; ?G; 7);
impl_tuple!(A, B, C, D, E, F, G, H,; ?H; 8);
impl_tuple!(A, B, C, D, E, F, G, H, I,; ?I; 9);
impl_tuple!(A, B, C, D, E, F, G, H, I, J,; ?J; 10);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K,; ?K; 11);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L,; ?L; 12);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M,; ?M; 13);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N,; ?N; 14);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,; ?O; 15);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,; ?P; 16);

/// Returns the length of a tuple.
#[inline]
pub const fn len<T>() -> usize
where
    T: Tuple,
{
    T::LEN
}

/// Returns the length of a tuple, by reference.
#[inline]
pub const fn len_ref<T>(tuple: &T) -> usize
where
    T: Tuple,
{
    // hush!
    let _tuple = tuple;

    T::LEN
}

/// Returns a reference to an element within a tuple at index `N`.
#[inline]
pub const fn get<const N: usize, T>(tuple: &T) -> &<T as TupleIndex<N>>::Element
where
    T: ~const TupleIndex<N>,
{
    T::get(tuple)
}

/// Returns a mutable reference to an element within a tuple at index `N`.
#[inline]
pub const fn get_mut<const N: usize, T>(tuple: &mut T) -> &mut <T as TupleIndex<N>>::Element
where
    T: ~const TupleIndex<N>,
{
    T::get_mut(tuple)
}
