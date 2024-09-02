//! This module implements traitsr from Alga to objects from faer.

use faer::{ComplexField, Entity, Row, Scale};

use super::base::{AlgebraField, DumDiv, FiniteVectorSpace};

impl<E: ComplexField> DumDiv<Scale<E>> for E {
    type Output = Self;

    fn dum_div(self, rhs: Scale<E>) -> Self::Output {
        self * rhs.0.faer_inv()
    }

    fn dum_div_assign(&mut self, rhs: Scale<E>) {
        *self *= rhs.0.faer_inv();
    }
}

impl<E: ComplexField> DumDiv<Scale<E>> for Scale<E> {
    type Output = Self;

    fn dum_div(self, rhs: Scale<E>) -> Self::Output {
        Scale(self.0 * rhs.0.faer_inv())
    }

    fn dum_div_assign(&mut self, rhs: Scale<E>) {
        *self = Scale(self.0 * rhs.0.faer_inv());
    }
}

impl<E: Entity + AlgebraField + ComplexField> FiniteVectorSpace for Row<E> {
    type Field = Scale<E>;
}
