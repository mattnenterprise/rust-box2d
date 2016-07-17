use super::super::super::common::Vec2;
use super::super::super::common::Transform;
use super::super::ray_cast_input::RayCastInput;
use super::super::ray_cast_output::RayCastOutput;
use super::super::aabb::AABB;
use super::mass_data::MassData;

pub enum ShapeType {
    Circle,
    Edge,
    Polygon,
    Chain
}

/// A shape is used for collision detection. You can create a shape however you like.
/// Shapes used for simulation in b2World are created automatically when a b2Fixture
/// is created. Shapes may encapsulate a one or more child shapes.
pub trait Shape {
    /// Get the type of this shape. You can use this to down cast to the concrete shape.
    /// @return the shape type.
    fn get_type(&self) -> ShapeType;

    /// Get the number of child primitives.
    fn get_child_count(&self) -> i32;

    /// Test a point for containment in this shape. This only works for convex shapes.
    /// @param xf the shape world transform.
    /// @param p a point in world coordinates.
    fn test_point(&self, xf: Transform, p: Vec2) -> bool;

    /// Cast a ray against a child shape.
	/// @param output the ray-cast results.
	/// @param input the ray-cast input parameters.
	/// @param transform the transform to be applied to the shape.
	/// @param child_index the child shape index
    fn ray_cast(&self, input: RayCastInput, transform: Transform, child_index: i32) -> (RayCastOutput, bool);

    /// Given a transform, compute the associated axis aligned bounding box for a child shape.
	/// @param xf the world transform of the shape.
	/// @param child_index the child shape.
    fn compute_aabb(&self, xf: Transform, child_index: i32) -> AABB;

    /// Compute the mass properties of this shape using its dimensions and density.
	/// The inertia tensor is computed about the local origin.
	/// @param density the density in kilograms per meter squared.
    fn compute_mass(&self, density: f32) -> MassData;
}
