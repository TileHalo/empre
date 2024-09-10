//! Empre prelude contains both abstract algebra and numerical derivation and
//! integration

#[cfg(feature = "faer")]
pub mod faer_vec;

#[cfg(feature = "nalgebra")]
pub mod nalgebra_vec;

pub mod base;
pub mod coordinate;
pub mod derivative;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// [Alias] Trait alias of Addition
pub trait Addition<Rhs = Self>: Sized + Add<Rhs, Output = Self> + AddAssign<Rhs> {}
/// [Alias] Trait alias of Subtraction
pub trait Subtraction<Rhs = Self>: Sized + Sub<Rhs, Output = Self> + SubAssign<Rhs> {}
/// [Alias] Trait alias of Multiplication
pub trait Multiplication<Rhs = Self>: Sized + Mul<Rhs, Output = Self> + MulAssign<Rhs> {}
/// [Alias] Trait alias for Division
pub trait Division<Rhs = Self>: Sized + Div<Rhs, Output = Self> + DivAssign<Rhs> {}

/// [alias] Trait alias for division and multiplication
pub trait DivMul<Rhs = Self>: Multiplication<Rhs> + Division<Rhs> {}

/// Trait for types that can be conjugated.
pub trait Conjugate {
    /// Conjugation operation
    fn conj(self) -> Self;
}

/// Trait that provides zero element
pub trait Zero {
    /// Provides the zero
    fn zero() -> Self;
}

/// [Alias] The Base field type
pub trait AlgebraField:
    Addition + Subtraction + Multiplication + Division + Zero + Clone + Copy + Conjugate
{
}
/// [Alias] The Base field type
pub trait AlgebraRing: Addition + Subtraction + Multiplication {}

/// The basic vector space abstraction
pub trait FiniteVectorSpace<T: AlgebraField>:
    Addition<Self> + Subtraction<Self> + Multiplication<T> + Division<T> + Zero + Clone
{
}

/// Inner product space
pub trait InnerProductSpace<T: AlgebraField>: FiniteVectorSpace<T> {
    /// Dot product
    fn dot(self, rhs: Self) -> T;
}

/// Inner product space
pub trait CrossProductSpace<T: AlgebraField>: FiniteVectorSpace<T> {
    /// Dot product
    fn cross(self, rhs: Self) -> Self;
}

/// Marker for three dimensional vector space
pub trait ThreeDVectorSpace<T: AlgebraField>:
    FiniteVectorSpace<T> + InnerProductSpace<T> + CrossProductSpace<T>
{
}

impl<U, T: Add<U, Output = Self> + AddAssign<U>> Addition<U> for T {}
impl<U, T: Sub<U, Output = Self> + SubAssign<U>> Subtraction<U> for T {}
impl<U, T: Mul<U, Output = Self> + MulAssign<U>> Multiplication<U> for T {}
impl<U, T: Div<U, Output = Self> + DivAssign<U>> Division<U> for T {}

impl<T: Addition + Subtraction + Multiplication + Division + Zero + Clone + Copy + Conjugate>
    AlgebraField for T
{
}
impl<T: Addition + Subtraction + Multiplication> AlgebraRing for T {}

macro_rules! impl_conjugate_reals {
    ($type:ty) => {
        impl Conjugate for $type {
            fn conj(self) -> Self {
                self
            }
        }
    };
}

impl_conjugate_reals!(f64);
impl_conjugate_reals!(f32);
