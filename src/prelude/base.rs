//! Prelude base, contains various (abstract) algebra traits.
use std::ops::{Div, DivAssign, Add, AddAssign, Mul, MulAssign, Sub, SubAssign};


/// [\Alias] Trait alias of Addition
pub trait Addition<'a, 'b, Rhs = Self>: Sized + Add<Rhs, Output = Self> + AddAssign<Rhs>
where &'a Self: Add<&'b Rhs> + AddAssign<&'b Rhs>, Self: 'a, Rhs: 'b{}
/// [\Alias] Trait alias of Subtraction
pub trait Subtraction<'a, 'b, Rhs = Self>: Sized + Sub<Rhs, Output = Self> + SubAssign<Rhs>
where &'a Self: Sub<&'b Rhs> + SubAssign<&'b Rhs>, Self: 'a, Rhs: 'b{}
/// [\Alias] Trait alias of Multiplication
pub trait Multiplication<'a, 'b, Rhs = Self>: Sized + Mul<Rhs, Output = Self> + MulAssign<Rhs>
where &'a Self: Mul<&'b Rhs> + MulAssign<&'b Rhs>, Self: 'a, Rhs: 'b{}

/// [\Alias] Trait alias of Division
pub trait Division<'a, 'b, Rhs = Self>: Sized + Div<Rhs, Output = Self> + DivAssign<Rhs>
where &'a Self: Div<&'b Rhs> + DivAssign<&'b Rhs>, Self: 'a, Rhs: 'b{}

/// Trait that guarantees a zero element
pub trait Zero {
    /// Zero Value
    fn zero() -> Self;
}

/// Trait that guarantees an identity element
pub trait One {
    /// Identity Value
    fn one() -> Self;
}

/// [\Alias] The Base field type
pub trait AlgebraField<'a>: Addition<'a, 'a> + Subtraction<'a, 'a> + Multiplication<'a, 'a> + Division<'a, 'a> + Zero + One + Clone + Copy
where &'a Self: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign, Self: 'a{}

/// The basic vector space abstraction
pub trait FiniteVectorSpace<'a, 'b, T: AlgebraField<'b>>:
    Addition<'a, 'a, Self> + Subtraction<'a, 'a, Self> + Multiplication<'a, 'b,T> + Division<'a, 'b,T> + Zero
where &'a Self: Add + AddAssign + Sub + SubAssign + Mul<&'b T> + MulAssign<&'b T> + Div<&'b T> + DivAssign<&'b T>,
&'b T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign, Self: 'a, T: 'b{}

impl <'a, 'b, T: Sized + Add<U, Output = T> + AddAssign<U>, U>  Addition<'a, 'b, U>
    for T
where &'a T: Add<&'b U> + AddAssign<&'b U>, T: 'a, U: 'b{}

impl <'a, 'b, T: Sized + Sub<U, Output = T> + SubAssign<U>, U>  Subtraction<'a, 'b, U>
    for T
where &'a T: Sub<&'b U> + SubAssign<&'b U>, T: 'a, U: 'b{}
impl <'a, 'b, T: Sized + Mul<U, Output = T> + MulAssign<U>, U>  Multiplication<'a, 'b, U>
    for T
where &'a T: Mul<&'b U> + MulAssign<&'b U>, T: 'a, U: 'b{}
impl <'a, 'b, T: Sized + Div<U, Output = T> + DivAssign<U>, U>  Division<'a, 'b, U>
    for T
where &'a T: Div<&'b U> + DivAssign<&'b U>, T: 'a, U: 'b{}

impl <'a, T:  Addition<'a, 'a> + Subtraction<'a, 'a> + Multiplication<'a, 'a> + Division<'a, 'a> + Zero + One + Clone + Copy>  AlgebraField<'a>
    for T
where &'a T: Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Div + DivAssign, T: 'a{}
