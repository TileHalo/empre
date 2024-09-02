use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[cfg(not(feature = "dumdiv"))]
use std::ops::{Div, DivAssign};

use alga::general::{ClosedAdd, ClosedDiv, ClosedMul, ClosedSub};

#[cfg(feature = "dumdiv")]
/// A Dummy trait that implements Div and DivAssign for objects not implementing
/// division (but might provide a way to do it).
pub trait DumDiv<Rhs = Self> {
    /// Output type
    type Output;
    /// The dummy division
    fn dum_div(self, rhs: Rhs) -> Self::Output;
    /// The dummy div assign
    fn dum_div_assign(&mut self, rhs: Rhs);
}

/// [Alias] Trait alias of Addition
pub trait Addition<Rhs = Self>: Sized + Add<Rhs, Output = Self> + AddAssign<Rhs> {}
/// [Alias] Trait alias of Subtraction
pub trait Subtraction<Rhs = Self>: Sub<Rhs, Output = Self> + SubAssign<Rhs> {}
/// [Alias] Trait alias of Multiplication
pub trait Multiplication<Rhs = Self>: Sized + Mul<Rhs, Output = Self> + MulAssign<Rhs> {}

#[cfg(not(feature = "dumdiv"))]
/// [Alias] Trait alias of Division
pub trait Division<Rhs = Self>: Sized + Div<Rhs, Output = Self> + DivAssign<Rhs> {}

#[cfg(feature = "dumdiv")]
/// [Alias] Trait alias of Division
pub trait Division<Rhs = Self>: Sized + DumDiv<Rhs> {}

pub trait Zero {
    fn zero() -> Self;
}

/// [Alias] The Base field type
pub trait AlgebraField: Addition + Subtraction + Multiplication + Division + Zero {}
/// [Alias] The Base field type
pub trait AlgebraRing: Addition + Subtraction + Multiplication {}

/// The basic vector space abstraction
pub trait FiniteVectorSpace<T: AlgebraField>:
    ClosedAdd<Self> + ClosedSub<Self> + ClosedMul<T> + ClosedDiv<T> + Zero
{
}

impl<T: Add<Self, Output = Self> + AddAssign<Self>> Addition for T {}
impl<T: Sub<Self, Output = Self> + SubAssign<Self>> Subtraction for T {}
impl<T: Mul<Self, Output = Self> + MulAssign<Self>> Multiplication for T {}
#[cfg(not(feature = "dumdiv"))]
impl<T: Div<Self, Output = Self> + DivAssign<Self>> Division for T {}
#[cfg(feature = "dumdiv")]
impl<T: DumDiv> Division for T {}

impl<T: Addition + Subtraction + Multiplication + Division + Zero> AlgebraField for T {}
impl<T: Addition + Subtraction + Multiplication> AlgebraRing for T {}
