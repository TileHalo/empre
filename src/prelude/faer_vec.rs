//! This module implements traitsr from Alga to objects from faer.

use faer::{ComplexField, Entity, Row, Scale};

use super::base::{AlgebraField, FiniteVectorSpace, Zero};


impl<E: ComplexField> Zero for Scale<E> {
    fn zero() -> Self {
        Scale(E::faer_zero())
    }
}

impl<E: Entity + ComplexField> Zero for Row<E> {
    fn zero() -> Self {
        Row::zeros(1)
    }
}

impl<E: Entity + AlgebraField + ComplexField> FiniteVectorSpace<Scale<E>> for Row<E> {}
