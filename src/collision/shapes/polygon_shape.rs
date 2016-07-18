use super::super::super::common::settings::{POLYGON_RADIUS, MAX_POLYGON_VERTICES};
use super::super::super::common::Vec2;
use super::super::super::common::Transform;
use super::super::super::common::math::{mul_rot_vec2, mul_rot_vec2_inverse, mul_transform_vec2, min_vec, max_vec};
use super::super::ray_cast_input::RayCastInput;
use super::super::ray_cast_output::RayCastOutput;
use super::super::aabb::AABB;
use super::shape::{Shape, ShapeType};
use super::mass_data::MassData;

/// A convex polygon. It is assumed that the interior of the polygon is to
/// the left of each edge.
/// Polygons have a maximum number of vertices equal to b2_maxPolygonVertices.
/// In most cases you should not need many vertices for a convex polygon.
pub struct PolygonShape {
    pub m_centroid: Vec2,
    pub m_vertices: [Vec2; MAX_POLYGON_VERTICES as usize],
    pub m_normals: [Vec2; MAX_POLYGON_VERTICES as usize],
    pub m_count: i32,
    pub m_radius: f32,
}

impl PolygonShape {
    pub fn new() -> PolygonShape {
        PolygonShape {
            m_centroid: Vec2::zero(),
            m_vertices: [Vec2::zero(); MAX_POLYGON_VERTICES as usize],
            m_normals: [Vec2::zero(); MAX_POLYGON_VERTICES as usize],
            m_count: 0,
            m_radius: POLYGON_RADIUS,
        }
    }
}

impl Shape for PolygonShape {
    /// Get the type of this shape. You can use this to down cast to the concrete shape.
    /// @return the shape type.
    fn get_type(&self) -> ShapeType {
        ShapeType::Polygon
    }

    /// Get the number of child primitives.
    fn get_child_count(&self) -> i32 {
        1
    }

    /// Test a point for containment in this shape. This only works for convex shapes.
    /// @param xf the shape world transform.
    /// @param p a point in world coordinates.
    fn test_point(&self, xf: Transform, p: Vec2) -> bool {
        // TODO implement
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
