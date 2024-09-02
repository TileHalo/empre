//! Empre prelude contains both abstract algebra and numerical derivation and
//! integration

#[cfg(feature = "faer")]
pub mod faer_vec;

#[cfg(feature = "nalgebra")]
pub mod nalgebra_vec;

pub mod base;
pub mod coordinate;
pub mod derivative;

pub use base::FiniteVectorSpace;
