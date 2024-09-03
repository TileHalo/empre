//! This module implements traitsr from Alga to objects from faer.

use faer::{ComplexField, Entity, Row, Scale};

use super::base::{AlgebraField, FiniteVectorSpace, One, Zero};


impl<E: ComplexField> Zero for Scale<E> {
    fn zero() -> Self {
        Scale(E::faer_zero())
    }
}

impl<E: ComplexField> One for Scale<E> {
    fn one() -> Self {
        Scale(E::faer_one())
    }
}

impl<E: Entity + ComplexField> Zero for Row<E> {
    fn zero() -> Self {
        Row::zeros(1)
    }
}

impl<'a, 'b, E: Entity + AlgebraField<'b> + ComplexField> FiniteVectorSpace<'a, 'b, Scale<E>> for Row<E> {}
