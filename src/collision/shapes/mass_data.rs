use super::super::super::common::Vec2;

/// The holds the mass data computed for a shape.
pub struct MassData {
    /// The mass of the shape, usually in kilograms.
    pub mass: f32,
    /// The position of the shape's centroid relative to the shape's origin.
    pub center: Vec2,
    /// The rotational interia of the shape about the local origin.
    pub I: f32
}

impl MassData {
    fn new() -> MassData {
        MassData {
            mass: 0.0,
            center: Vec2::zero(),
            I: 0.0
        }
    }
}
