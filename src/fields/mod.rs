//! This module contains both electromagnetic fields and various auxiliary fields (together with
//! the definition of both vector and scalar fields).

use crate::prelude::{AlgebraField, FiniteVectorSpace};


pub mod scalar;
pub mod vector;

/// The base field, all scalar/vector/tensor etc. fields should implement this.
pub trait ElementField<T: AlgebraField, V, const N: usize> {
    /// The coordinates for which Input values correspond to.
    type Space;
    /// The input domain. For analytical (and continuous) domains this is the same as space.
    type Domain: From<[T; N]>;

    /// Coordinates at point, mainly used when dealing with discrete spaces.
    fn coordinates(&self, x: Self::Domain) -> Self::Space;
    /// Value at point
    fn value(&self, x: Self::Domain) -> V;
}

/// The base scalar field, a function f(S) -> F where F is the scalar field
pub trait ScalarField<T: AlgebraField, D, const N: usize>: ElementField<T, T, N> {
    /// Gradient of the scalar field
    fn grad<V: FiniteVectorSpace<T>>(&self, diff: D) -> impl VectorField<T, D, V, N>;
    /// Laplacian of the scalar field
    fn laplacian(&self, diff: D) -> impl ScalarField<T, D, N>;
}

/// The base vector field
pub trait VectorField<T: AlgebraField, D, V: FiniteVectorSpace<T>, const N: usize>:
    ElementField<T, V, N> + FiniteVectorSpace<T>
{
    /// Divergence of the vector field
    fn divergence<P>(&self, diff: D) -> impl ScalarField<T, P, N>;
    /// Gradient of the vector field
    fn curl(&self, diff: D) -> impl VectorField<T, D, V, N>;
    /// Gradient of the vector field
    fn laplacian(&self, diff: D) -> impl VectorField<T, D, V, N>;

    /// Dot product with vector
    fn dot(&self, rhs: V) -> impl ScalarField<T, D, N>;

    /// Dot product with vector field
    fn dotf(&self, rhs: Self) -> impl ScalarField<T, D, N>;

    /// Cross product with vector
    fn cross(&self, rhs: V) -> impl VectorField<T, D, V, N>;

    /// Dot product with vector
    fn crossf(&self, rhs: Self) -> impl VectorField<T, D, V, N>;

    /// Vector field magnitude
    fn mag(&self) -> impl ScalarField<T, D, N>;
}
