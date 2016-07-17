use super::super::super::common::settings::POLYGON_RADIUS;
use super::super::super::common::Vec2;
use super::super::super::common::Transform;
use super::super::super::common::math::mul_rot_vec2;
use super::super::ray_cast_input::RayCastInput;
use super::super::ray_cast_output::RayCastOutput;
use super::super::aabb::AABB;
use super::shape::{Shape, ShapeType};
use super::mass_data::MassData;

pub struct EdgeShape {
    /// Edge vertex
    pub m_vertex1: Vec2,
    /// Edge vertex
    pub m_vertex2: Vec2,
    /// Optional adjacent vertex used for smooth collision.
    pub m_vertex0: Vec2,
    /// Optional adjacent vertex used for smooth collision.
    pub m_vertex3: Vec2,
    pub m_has_vertex0: bool,
    pub m_has_vertex3: bool,
    pub m_radius: f32
}

impl EdgeShape {
    pub fn new() -> EdgeShape {
        EdgeShape {
            m_vertex1: Vec2::zero(),
            m_vertex2: Vec2::zero(),
            m_vertex0: Vec2::zero(),
            m_vertex3: Vec2::zero(),
            m_has_vertex0: false,
            m_has_vertex3: false,
            m_radius: POLYGON_RADIUS
        }
    }
}

impl Shape for EdgeShape {
    /// Get the type of this shape. You can use this to down cast to the concrete shape.
    /// @return the shape type.
    fn get_type(&self) -> ShapeType {
        ShapeType::Edge
    }

    /// Get the number of child primitives.
    fn get_child_count(&self) -> i32 {
        1
    }

    /// Test a point for containment in this shape. This only works for convex shapes.
    /// @param xf the shape world transform.
    /// @param p a point in world coordinates.
    fn test_point(&self, xf: Transform, p: Vec2) -> bool {
        false
    }

    /// Cast a ray against a child shape.
	/// @param output the ray-cast results.
	/// @param input the ray-cast input parameters.
	/// @param transform the transform to be applied to the shape.
	/// @param child_index the child shape index
    fn ray_cast(&self, input: RayCastInput, transform: Transform, child_index: i32) -> Option<RayCastOutput> {
        // TODO implement
        None
    }

    /// Given a transform, compute the associated axis aligned bounding box for a child shape.
	/// @param xf the world transform of the shape.
	/// @param child_index the child shape.
    fn compute_aabb(&self, xf: Transform, child_index: i32) -> AABB {
        // TODO implement
        AABB::new()
    }

    /// Compute the mass properties of this shape using its dimensions and density.
	/// The inertia tensor is computed about the local origin.
	/// @param density the density in kilograms per meter squared.
    fn compute_mass(&self, density: f32) -> MassData {
        // TODO implement
        MassData::new()
    }

    /// Get the radius. Regular box2d has this as a base class field.
    fn get_radius(&self) -> f32 {
        self.m_radius
    }

    /// Set the radius. Regular box2d has this as a base class field.
    fn set_radius(&mut self, r: f32) {
        self.m_radius = r;
    }
}
