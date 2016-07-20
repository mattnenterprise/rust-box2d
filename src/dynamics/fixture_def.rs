use super::super::collision::shape::Shape;
use super::filter::Filter;

/// A fixture definition is used to create a fixture. This class defines an
/// abstract fixture definition. You can reuse fixture definitions safely.
/// TODO Should I be using generics ?
pub struct FixtureDef<T: Shape> {
    /// The shape, this must be set. The shape will be cloned, so you
	/// can create the shape on the stack.
    pub shape: T,
    // TODO user_data
    /// The friction coefficient, usually in the range [0,1].
    pub friction: f32
    /// The restitution (elasticity) usually in the range [0,1].
    pub restitution: f32,
    /// The density, usually in kg/m^2.
    pub density: f32,
    /// A sensor shape collects contact information but never generates a collision
	/// response.
    pub is_sensor: bool,
    pub filter: Filter
}

impl<T: Shape> FixtureDef<T> {
    fn new<T: Shape>(shape: T) -> FixtureDef<Shape> {
        FixtureDef {
            shape: shape,
            // TODO user_data
            friction: 0.2,
            restitution: 0.0,
            density: 0.0,
            is_sensor: false,
            filter: Filter::new()
        }
    }
}
