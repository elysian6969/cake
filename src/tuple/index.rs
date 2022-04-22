//! TupleIndex trait and implementation.

use super::Tuple;

/// A tuple element, at index `N`.
pub trait TupleIndex<const N: usize>: Tuple {
    type Element;

    const INDEX: usize;

    fn get(&self) -> &Self::Element;
    fn get_mut(&mut self) -> &mut Self::Element;
}

macro_rules! impl_tuple_index {
    (($($element:ident,)*); $which:ident[$index:literal]; $self:ident.$get:tt) => {
        impl<$($element,)*> const TupleIndex<$index> for ($($element,)*) {
            type Element = $which;

            const INDEX: usize = $index;

            fn get(&$self) -> &$which {
                &$self.$get
            }

            fn get_mut(&mut $self) -> &mut $which {
                &mut $self.$get
            }
        }
    }
}

impl_tuple_index!((A,); A[0]; self.0);

impl_tuple_index!((A, B,); A[0]; self.0);
impl_tuple_index!((A, B,); B[1]; self.1);

impl_tuple_index!((A, B, C,); A[0]; self.0);
impl_tuple_index!((A, B, C,); B[1]; self.1);
impl_tuple_index!((A, B, C,); C[2]; self.2);

impl_tuple_index!((A, B, C, D,); A[0]; self.0);
impl_tuple_index!((A, B, C, D,); B[1]; self.1);
impl_tuple_index!((A, B, C, D,); C[2]; self.2);
impl_tuple_index!((A, B, C, D,); D[3]; self.3);

impl_tuple_index!((A, B, C, D, E,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E,); E[4]; self.4);

impl_tuple_index!((A, B, C, D, E, F,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F,); F[5]; self.5);

impl_tuple_index!((A, B, C, D, E, F, G,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G,); G[6]; self.6);

impl_tuple_index!((A, B, C, D, E, F, G, H,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H,); H[7]; self.7);

impl_tuple_index!((A, B, C, D, E, F, G, H, I,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I,); I[8]; self.8);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J,); J[9]; self.9);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); J[9]; self.9);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K,); K[10]; self.10);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); J[9]; self.9);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); K[10]; self.10);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L,); L[11]; self.11);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); J[9]; self.9);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); K[10]; self.10);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); L[11]; self.11);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M,); M[12]; self.12);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); J[9]; self.9);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); K[10]; self.10);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); L[11]; self.11);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); M[12]; self.12);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N,); N[13]; self.13);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); J[9]; self.9);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); K[10]; self.10);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); L[11]; self.11);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); M[12]; self.12);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); N[13]; self.13);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,); O[14]; self.14);

impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); A[0]; self.0);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); B[1]; self.1);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); C[2]; self.2);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); D[3]; self.3);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); E[4]; self.4);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); F[5]; self.5);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); G[6]; self.6);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); H[7]; self.7);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); I[8]; self.8);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); J[9]; self.9);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); K[10]; self.10);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); L[11]; self.11);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); M[12]; self.12);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); N[13]; self.13);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); O[14]; self.14);
impl_tuple_index!((A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,); P[15]; self.15);
