//! Base scalar field module. Contains an implementation for discrete scalar field.
use alga::general::Field;
use ndarray::{Array, Dim, Ix};

use crate::prelude::{base::AlgebraField, FiniteVectorSpace};

use super::{vector::ZeroVectorField, ElementField, ScalarField, VectorField};

/// Base N-dimensional scalar field
#[derive(Clone)]
pub struct DiscreteScalarField<T: Field, V: Field, const N: usize> {
    /// Coordinates of the scalar field
    pub coords: Array<[T; N], Dim<[Ix; N]>>,
    /// Values of the scalar field
    pub data: Array<V, Dim<[Ix; N]>>,
}

/// A Zero scalar field, i.e. everything is zero everywhere

#[derive(Clone, Copy)]
pub struct ZeroScalarField;

impl<T: AlgebraField, const N: usize> ElementField<T, T, N> for ZeroScalarField {
    type Space = ();
    type Domain = [T; N];

    fn coordinates(&self, _: Self::Domain) -> Self::Space {
        ()
    }

    fn value(&self, _: Self::Domain) -> T {
        T::zero()
    }
}

impl<T: AlgebraField, D, const N: usize> ScalarField<T, D, N> for ZeroScalarField {
    fn grad<V: FiniteVectorSpace<T>>(&self, _: D) -> impl VectorField<T, D, V, N> {
        ZeroVectorField
    }
    fn laplacian(&self, _: D) -> impl ScalarField<T, D, N> {
        ZeroScalarField
    }
}
