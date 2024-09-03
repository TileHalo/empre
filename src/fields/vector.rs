//! Base vector field module. Contains an implementation for discrete vector field.
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::prelude::{
    base::{AlgebraField, Zero},
    coordinate::CoordinateSystem,
    FiniteVectorSpace,
};

use super::{scalar::ZeroScalarField, ElementField, ScalarField, VectorField};

/// Modifier for for whether curl is zero or not
pub trait CurlModifier: Copy + Clone {}
/// Modifier for for whether div is zero or not
pub trait DivModifier: Copy + Clone {}

/// Marker for field type
pub trait FieldType: Copy + Clone {}

/// Marker for conservative (i.e. curl free) vector field
#[derive(Copy, Clone)]
pub struct ConservativeField;

/// Marker for divergence free vector field.
#[derive(Copy, Clone)]
pub struct DivFreeField;

/// Marker for a field where no information on curl or divergence exists.
#[derive(Copy, Clone)]
pub struct AnyField;

/// A Zero vector field, i.e. everything is zero everywhere
#[derive(Clone, Copy)]
pub struct ZeroVectorField;

/// Base N-dimensional vector field
#[derive(Clone)]
pub struct DiscreteVectorField<
    T: AlgebraField + Default,
    V: FiniteVectorSpace<T> + Default,
    const N: usize,
    A: CurlModifier + DivModifier,
    C: CoordinateSystem,
    F: FieldType,
> {
    /// The coordinate data
    pub coords: Vec<[T; N]>,
    /// Values of the scalar field
    pub data: Vec<V>,
    /// Shape of the vector field
    pub shape: [usize; N],
    _a: PhantomData<A>,
    _c: PhantomData<C>,
    _f: PhantomData<F>,
}

impl CurlModifier for ConservativeField {}
impl CurlModifier for DivFreeField {}
impl CurlModifier for AnyField {}

impl DivModifier for ConservativeField {}
impl DivModifier for DivFreeField {}
impl DivModifier for AnyField {}

impl FieldType for AnyField {}

impl<T: AlgebraField + Default, const N: usize, V: FiniteVectorSpace<T> + Default, C: CoordinateSystem>
    DiscreteVectorField<T, V, N, AnyField, C, AnyField>
{
    /// Create a new empty vector field
    pub fn new() -> Self {
        DiscreteVectorField {
            coords: Vec::new(),
            data: Vec::new(),
            shape: [0; N],
            _a: PhantomData,
            _c: PhantomData,
            _f: PhantomData,
        }
    }
}

impl<'a,
        T: AlgebraField + Default,
        V: FiniteVectorSpace<T> + Default,
        const N: usize,
        A: CurlModifier + DivModifier,
        C: CoordinateSystem,
        F: FieldType,
    > Add for DiscreteVectorField<T, V, N, A, C, F>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        DiscreteVectorField {
            coords: self.coords.iter().zip(rhs.coords.iter()).map(|(a, _)| *a).collect(),
            data: self.data.iter().zip(&rhs.data).map(|(a, b)| a + b).collect(),
            shape: self.shape,
            _a: self._a,
            _c: self._c,
            _f: self._f,
        }
    }
}
impl<
        T: AlgebraField + Default,
        V: FiniteVectorSpace<T> + Default,
        const N: usize,
        A: CurlModifier + DivModifier,
        C: CoordinateSystem,
        F: FieldType,
    > Sub for DiscreteVectorField<T, V, N, A, C, F>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        DiscreteVectorField {
            coords: self.coords.iter().zip(rhs.coords.iter()).map(|(a, _)| a.clone()).collect(),
            data: self.data.iter().zip(rhs.data.into_iter()).map(|(a, b)| a.clone() - b).collect(),
            shape: self.shape,
            _a: self._a,
            _c: self._c,
            _f: self._f,
        }
    }
}
impl<
        T: AlgebraField + Default,
        V: FiniteVectorSpace<T> + Default,
        const N: usize,
        A: CurlModifier + DivModifier,
        C: CoordinateSystem,
        F: FieldType,
    > Mul<T> for DiscreteVectorField<T, V, N, A, C, F>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        DiscreteVectorField {
            coords: self.coords,
            data: self.data.iter().map(|a| a.clone() * rhs.clone()).collect(),
            shape: self.shape,
            _a: self._a,
            _c: self._c,
            _f: self._f,
        }
    }
}

impl<
        T: AlgebraField + Default,
        V: FiniteVectorSpace<T> + Default,
        const N: usize,
        A: CurlModifier + DivModifier,
        C: CoordinateSystem,
        F: FieldType,
    > Div<T> for DiscreteVectorField<T, V, N, A, C, F>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        DiscreteVectorField {
            coords: self.coords,
            data: self.data.iter().map(|a| a.clone() / rhs.clone()).collect(),
            shape: self.shape,
            _a: self._a,
            _c: self._c,
            _f: self._f,
        }
    }
}

// Below this are ZeroVectorField impls, these are mostly boilerplate
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

impl Zero for ZeroVectorField {
    fn zero() -> Self {
        ZeroVectorField
    }
}
