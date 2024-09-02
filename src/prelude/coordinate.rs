//! Various coordinate systems and their markers

/// Marker trait for coordinate systems
pub trait CoordinateSystem: Copy + Clone {}

/// Standard cartesian coordinates
#[derive(Debug, Copy, Clone)]
pub struct CartesianCoordinates;

impl CoordinateSystem for CartesianCoordinates {}
