use alga::{general::Field, linear::VectorSpace};
use ndarray::{Array, Dim, Ix};

/// Base N-dimensional scalar field
#[derive(Clone)]
pub struct DiscreteScalarField<T: Field, V: Field, const N: usize> {
    /// Coordinates of the scalar field
    pub coords: Array<[T;N], Dim<[Ix; N]>>,
    /// Values of the scalar field
    pub data: Array<V, Dim<[Ix; N]>>,
}

/// Base N-dimensional scalar field
#[derive(Clone)]
pub struct DiscreteVectorField<T: Field, V: VectorSpace, const N: usize> {
    /// Coordinates of the scalar field
    pub coords: Array<[T;N], Dim<[Ix; N]>>,
    /// Values of the scalar field
    pub data: Array<V, Dim<[Ix; N]>>,
}
