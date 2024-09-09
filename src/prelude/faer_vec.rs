//! This module implements traitsr from Alga to objects from faer.

use faer::{prelude::{c32, c64}, Col, ComplexField, Entity, Row, Scale};
use num::Complex;

use super::{AlgebraField, Conjugate, FiniteVectorSpace, Zero};


impl<E: ComplexField> Zero for Scale<E> {
    fn zero() -> Self {
        Scale(E::faer_zero())
    }
}

impl<E: ComplexField> Zero for Row<E> {
    fn zero() -> Self {
        Row::zeros(1)
    }
}
impl<E: ComplexField> Zero for Col<E> {
    fn zero() -> Self {
        Col::zeros(1)
    }
}

impl  Conjugate for c32 {
    fn conj(self) -> Self {
        self.faer_conj()
    }
}

impl  Conjugate for c64 {
    fn conj(self) -> Self {
        self.faer_conj()
    }
}

impl<E: ComplexField> Conjugate for Complex<E> {
    fn conj(self) -> Self {
        Complex{ re: self.re, im: -self.im }
    }
}

impl <T: ComplexField> Conjugate for Scale<T> {
    fn conj(self) -> Self {
        Scale(self.0.faer_conj())
    }
}

impl<E: Entity + AlgebraField + ComplexField> FiniteVectorSpace<Scale<E>> for Row<E> {}
impl<E: Entity + AlgebraField + ComplexField> FiniteVectorSpace<Scale<E>> for Col<E> {}
