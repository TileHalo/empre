use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(not(feature = "dumdiv"))]
use std::ops::{Div, DivAssign};


/// [Alias] Trait alias of Addition
pub trait Addition<Rhs = Self>: Sized + Add<Rhs, Output = Self> + AddAssign<Rhs> {}
/// [Alias] Trait alias of Subtraction
pub trait Subtraction<Rhs = Self>: Sized + Sub<Rhs, Output = Self> + SubAssign<Rhs> {}
/// [Alias] Trait alias of Multiplication
pub trait Multiplication<Rhs = Self>: Sized + Mul<Rhs, Output = Self> + MulAssign<Rhs> {}

pub trait Division<Rhs = Self>: Sized + Div<Rhs, Output = Self> + DivAssign<Rhs> {}


pub trait Zero {
    fn zero() -> Self;
}

/// [Alias] The Base field type
pub trait AlgebraField: Addition + Subtraction + Multiplication + Division + Zero + Clone + Copy {}
/// [Alias] The Base field type
pub trait AlgebraRing: Addition + Subtraction + Multiplication {}

/// The basic vector space abstraction
pub trait FiniteVectorSpace<T: AlgebraField>:
    Addition<Self> + Subtraction<Self> + Multiplication<T> + Division<T> + Zero + Clone
{
}

impl<U, T: Add<U, Output = Self> + AddAssign<U>> Addition<U> for T {}
impl<U, T: Sub<U, Output = Self> + SubAssign<U>> Subtraction<U> for T {}
impl<U, T: Mul<U, Output = Self> + MulAssign<U>> Multiplication<U> for T {}
impl<U, T: Div<U, Output = Self> + DivAssign<U>> Division<U> for T {}

impl<T: Addition + Subtraction + Multiplication + Division + Zero + Clone + Copy> AlgebraField for T {}
impl<T: Addition + Subtraction + Multiplication> AlgebraRing for T {}
