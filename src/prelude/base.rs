//! Provides basic vector type
#![allow(clippy::needless_range_loop)]

use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};


use super::{AlgebraField, CrossProductSpace, FiniteVectorSpace, InnerProductSpace, Zero};


/// Base N-dimensional vector type
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector<T: AlgebraField, const N: usize>([T;N]);

/// Base 3-dimensional vector
pub type Vector3<T> = Vector<T, 3>;


impl <T: AlgebraField, const N: usize> Default for Vector<T, N> {
    /// Build new vector
     fn default() -> Self {
        Vector([T::zero();N])
    }
}

impl <T: AlgebraField, const N: usize> IndexMut<usize> for Vector<T, N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl <T: AlgebraField, const N: usize> Index<usize> for Vector<T, N> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl <T: AlgebraField, const N: usize> Zero for Vector<T, N> {
    fn zero() -> Self {
        Vector([T::zero();N])
    }
}


impl <T: AlgebraField, const N: usize> Add<Self> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut v = [T::zero();N];
        for i in 0..N {
            v[i] = self[i] + rhs[i];
        }
        Vector(v)
    }
}

impl <T: AlgebraField, const N: usize> AddAssign<Self> for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] += rhs[i];
        }
    }
}
impl <T: AlgebraField, const N: usize> Sub<Self> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut v = [T::zero();N];
        for i in 0..N {
            v[i] = self[i] - rhs[i];
        }
        Vector(v)
    }
}

impl <T: AlgebraField, const N: usize> SubAssign<Self> for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..N {
            self[i] -= rhs[i];
        }
    }
}

impl <T: AlgebraField, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut v = [T::zero();N];
        for i in 0..N {
            v[i] = self[i] * rhs;
        }
        Vector(v)
    }
}

impl <T: AlgebraField, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..N {
            self[i] *= rhs;
        }
    }
}
impl <T: AlgebraField, const N: usize> Div<T> for Vector<T, N> {
    type Output = Vector<T, N>;

    fn div(self, rhs: T) -> Self::Output {
        let mut v = [T::zero();N];
        for i in 0..N {
            v[i] = self[i] / rhs;
        }
        Vector(v)
    }
}

impl <T: AlgebraField, const N: usize> DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..N {
            self[i] /= rhs;
        }
    }
}

impl <T: AlgebraField, const N: usize> FiniteVectorSpace<T> for Vector<T, N> {}

impl <T: AlgebraField, const N: usize> InnerProductSpace<T> for Vector<T, N> {
    fn dot(self, rhs: Self) -> T {
        let mut v = T::zero();
        for i in 0..N {
            v += self[i] * rhs[i].conj();
        }
        v
    }
}

impl <T: AlgebraField> CrossProductSpace<T> for Vector<T, 3> {
    fn cross(self, rhs: Self) -> Self {
        Vector([self[1]*rhs[2] - self[2]*rhs[1], self[2]*rhs[0] - self[0]*rhs[2], self[0]*rhs[1] - self[1]*rhs[0]])
    }
}
