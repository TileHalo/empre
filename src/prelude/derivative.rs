//! Numerical differentiation lives here

use num::Num;
use std::ops::{Div, Index};

/// Any object that is differentiable must implement this
pub trait Differentiable {
    /// The input of the differentiable object. For example in discrete grids this is simply the
    /// index.
    type Input;
    /// The space (i.e. X in f: X -> Y) of the differentiable. In analytical functions this equals
    /// to Self::Input.
    type Space;
    /// The Singular coordinate type, ex. f64 where Space = (f64,f64)
    type Coordinate: Num + Copy;
    /// The resulting value (that is, Y in f: X -> Y).
    type Value: Num + Div<Self::Coordinate, Output = Self::Value>;

    /// The coordinates at the input (i.e. coordinates of the Self::Space).
    fn coord(&self, inp: Self::Input) -> Self::Space;
    /// The value at point.
    fn value(&self, inp: Self::Input) -> Self::Value;
    /// The value at point.
    fn input_range(&self) -> (Self::Input, Self::Input);
}

/// Base derivation trait.
pub trait Derivation<I: Differentiable, const D: usize> {
    /// The output of the derivation.
    type Output;
    /// The Derivation itself
    fn derivate_at(&self, input: I, coord: I::Input, nth: Option<usize>) -> Self::Output;
}

/// Discrete derivation that performs derivation on whole discrete space
pub struct DiscreteDerivative;
impl<
        I: Differentiable<Input = usize, Space: Num + Copy, Value: Div<I::Space, Output = I::Value>>,
    > Derivation<I, 1> for DiscreteDerivative
{
    type Output = Option<I::Value>;

    fn derivate_at(&self, input: I, coord: I::Input, _: Option<usize>) -> Self::Output {
        let (min, max) = input.input_range();
        let mut a = coord;
        let mut b = a;
        a += 1;
        b -= 1;
        if a > max {
            a = max;
        }
        if b < min {
            b = min;
        }
        let ac = input.coord(a);
        let bc = input.coord(b);
        let mc = input.coord(coord);

        let h = (ac - mc) + (mc - bc);

        Some((input.value(a) - input.value(b)) / h)
    }
}

macro_rules! n_dimensional_derivate {

    ($type:ty, ($($a:ty),*), $n:expr) => (
        impl <I: Differentiable<Input=($($a),*), Space: Index<usize, Output = I::Coordinate>>> Derivation<I, $n> for $type {

            type Output = Option<I::Value>;

            fn derivate_at(&self, input: I, coord: I::Input, nth: Option<usize>) -> Self::Output {
                match nth {
                    Some(n) => {
                    let (min_t, max_t) = input.input_range();
                    let max: [usize; $n]  = max_t.into();
                    let min: [usize; $n]  = min_t.into();
                    let mut a: [usize; $n] = coord.into();
                    let mut b = a;
                    a[n] += 1;
                    b[n] -= 1;
                    if a[n] > max[n] {
                        a[n] = max[n];
                    }
                    if b[n] < min[n] {
                        b[n] = min[n];
                    }
                    let at: ($($a),*) = a.into();
                    let bt: ($($a),*) = b.into();
                    let ac = input.coord(at);
                    let bc = input.coord(bt);
                    let mc = input.coord(coord);

                    let h = (ac[n] - mc[n]) + (mc[n] - bc[n]);

                    Some((input.value(at) - input.value(bt))/h)},
                    None => None
                }
            }
        }
    )
}

n_dimensional_derivate!(DiscreteDerivative, (usize, usize), 2);
n_dimensional_derivate!(DiscreteDerivative, (usize, usize, usize), 3);
n_dimensional_derivate!(DiscreteDerivative, (usize, usize, usize, usize), 4);
