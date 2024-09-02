//! This module contains both electromagnetic fields and various auxiliary fields (together with
//! the definition of both vector and scalar fields).

use crate::prelude::base::{AlgebraField, FiniteVectorSpace};

pub mod scalar;


/// The base field, all scalar/vector/tensor etc. fields should implement this.
pub trait ElementField<T: AlgebraField, const N: usize> {
    /// The coordinates for which Input values correspond to.
    type Space;
    /// The input domain. For analytical (and continuous) domains this is the same as space.
    type Domain: From<[T; N]>;
    /// The value type. Can be anything.
    type Value;

    /// Coordinates at point, mainly used when dealing with discrete spaces.
    fn coordinates(&self, x: Self::Domain) -> Self::Space;
    /// Value at point
    fn value(&self, x: Self::Domain) -> Self::Value;
}

/// The base scalar field, a function f(S) -> F where F is the scalar field
pub trait ScalarField<T: AlgebraField, D, const N: usize>: ElementField<T, N, Value = T> {
    /// Gradient of the scalar field
    fn grad<V: FiniteVectorSpace>(&self, diff: D) -> impl VectorField<T, D, N, V>;
    /// Laplacian of the scalar field
    fn laplacian(&self, diff: D) -> impl ScalarField<T, D, N>;
}
/// The base vector field
pub trait VectorField<T: AlgebraField, D, const N: usize, V: FiniteVectorSpace>:
    ElementField<T, N, Value = V> + FiniteVectorSpace
{
    /// Divergence of the vector field
    fn divergence<P>(&self, diff: D) -> impl ScalarField<T, P, N>;
    /// Gradient of the vector field
    fn curl(&self, diff: D) -> impl VectorField<T, D, N, V>;
    /// Gradient of the vector field
    fn laplacian(&self, diff: D) -> impl VectorField<T, D, N, V>;
}
