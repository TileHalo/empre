//! Base vector field module. Contains an implementation for discrete vector field.
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use alga::{general::Field, linear::VectorSpace};
use ndarray::{Array, Dim, Ix};

use crate::prelude::{
    base::{AlgebraField, DumDiv, Zero},
    FiniteVectorSpace,
};

use super::{scalar::ZeroScalarField, ElementField, ScalarField, VectorField};

/// Base N-dimensional vector field
#[derive(Clone)]
/// Coordinates of the vector field
pub struct DiscreteVectorField<T: Field, V: VectorSpace, const N: usize> {
    pub coords: Array<[T; N], Dim<[Ix; N]>>,
    /// Values of the scalar field
    pub data: Array<V, Dim<[Ix; N]>>,
}

/// A Zero vector field, i.e. everything is zero everywhere
#[derive(Clone, Copy)]
pub struct ZeroVectorField;

impl<T: AlgebraField> FiniteVectorSpace<T> for ZeroVectorField {}

impl<T: AlgebraField, V: FiniteVectorSpace<T>, const N: usize> ElementField<T, V, N>
    for ZeroVectorField
{
    type Space = ();
    type Domain = [T; N];

    fn coordinates(&self, _: Self::Domain) -> Self::Space {
        ()
    }

    fn value(&self, _: Self::Domain) -> V {
        V::zero()
    }
}
/// The base vector field
impl<T: AlgebraField, D, V: FiniteVectorSpace<T>, const N: usize> VectorField<T, D, V, N>
    for ZeroVectorField
{
    /// Divergence of the vector field
    fn divergence<P>(&self, _: D) -> impl ScalarField<T, P, N> {
        ZeroScalarField
    }
    /// Gradient of the vector field
    fn curl(&self, _: D) -> impl VectorField<T, D, V, N> {
        ZeroVectorField
    }
    /// Gradient of the vector field
    fn laplacian(&self, _: D) -> impl VectorField<T, D, V, N> {
        ZeroVectorField
    }

    /// Dot product with vector
    fn dot(&self, _: V) -> impl ScalarField<T, D, N> {
        ZeroScalarField
    }

    /// Dot product with vector field
    fn dotf(&self, _: Self) -> impl ScalarField<T, D, N> {
        ZeroScalarField
    }

    /// Cross product with vector
    fn cross(&self, _: V) -> impl VectorField<T, D, V, N> {
        ZeroVectorField
    }

    /// Dot product with vector
    fn crossf(&self, _: Self) -> impl VectorField<T, D, V, N> {
        ZeroVectorField
    }

    /// Vector field magnitude
    fn mag(&self) -> impl ScalarField<T, D, N> {
        ZeroScalarField
    }
}

// Implement all necessary but tedious stuff (boilerplate here)

impl<T> Add<T> for ZeroVectorField {
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        rhs
    }
}

impl<T> Sub<T> for ZeroVectorField {
    type Output = T;

    fn sub(self, rhs: T) -> Self::Output {
        rhs
    }
}

impl<T> Mul<T> for ZeroVectorField {
    type Output = ZeroVectorField;

    fn mul(self, _: T) -> Self::Output {
        self
    }
}

impl<T> Div<T> for ZeroVectorField {
    type Output = ZeroVectorField;

    fn div(self, _: T) -> Self::Output {
        self
    }
}

impl AddAssign<ZeroVectorField> for ZeroVectorField {
    fn add_assign(&mut self, _: Self) {}
}

impl SubAssign<ZeroVectorField> for ZeroVectorField {
    fn sub_assign(&mut self, _: Self) {}
}

impl<T> MulAssign<T> for ZeroVectorField {
    fn mul_assign(&mut self, _: T) {}
}

impl<T> DivAssign<T> for ZeroVectorField {
    fn div_assign(&mut self, _: T) {}
}

#[cfg(feature = "dumdiv")]
impl<T> DumDiv<T> for ZeroVectorField {
    type Output = ZeroVectorField;
    fn dum_div(self, _: T) -> Self {
        self
    }
    fn dum_div_assign(&mut self, _: T) {}
}

impl Zero for ZeroVectorField {
    fn zero() -> Self {
        ZeroVectorField
    }
}
