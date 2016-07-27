use super::super::super::common::settings::POLYGON_RADIUS;
use super::super::super::common::Vec2;
use super::super::super::common::Transform;
use super::super::super::common::math::{mul_rot_vec2, mul_rot_vec2_inverse, mul_transform_vec2, min_vec, max_vec};
use super::super::ray_cast_input::RayCastInput;
use super::super::ray_cast_output::RayCastOutput;
use super::super::aabb::AABB;
use super::shape::{Shape, ShapeType};
use super::to_derived_shape::ToDerivedShape;
use super::circle_shape::CircleShape;
use super::polygon_shape::PolygonShape;
use super::chain_shape::ChainShape;
use super::mass_data::MassData;

#[derive(Clone, Copy)]
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
    fn test_point(&self, _: Transform, _: Vec2) -> bool {
        false
    }

    /// Cast a ray against a child shape.
	/// @param output the ray-cast results.
	/// @param input the ray-cast input parameters.
	/// @param transform the transform to be applied to the shape.
    fn ray_cast(&self, input: RayCastInput, xf: Transform, _: i32) -> Option<RayCastOutput> {
        // Put the ray into the edge's frame of reference.
        let p1 = mul_rot_vec2_inverse(xf.q, input.p1 - xf.p);
        let p2 = mul_rot_vec2_inverse(xf.q, input.p2 - xf.p);
        let d = p2 - p1;

        let v1 = self.m_vertex1;
        let v2 = self.m_vertex2;
        let e = v2 - v1;
        let mut normal = Vec2::new(e.y, -e.x);
        normal.normalize();

        // q = p1 + t * d
        // dot(normal, q - v1) = 0
        // dot(normal, p1 - v1) + t * dot(normal, d) = 0
        let numerator = Vec2::dot(normal, v1 - p1);
        let denominator = Vec2::dot(normal, d);

        if denominator == 0.0 {
            return None;
        }

        let t = numerator / denominator;
        if t < 0.0 || input.max_fraction < t {
            return None;
        }

        let q = p1 + t * d;

        // q = v1 + s * r
        // s = dot(q - v1, r) / dot(r, r)
        let r = v2 - v1;
        let rr = Vec2::dot(r, r);
        if rr == 0.0 {
            return None;
        }

        let s = Vec2::dot(q - v1, r) / rr;
        if s < 0.0 || 1.0 < s {
            return None;
        }

        let mut output = RayCastOutput::new();
        output.fraction = t;
        if numerator > 0.0 {
            output.normal = - mul_rot_vec2(xf.q, normal);
        } else {
            output.normal = mul_rot_vec2(xf.q, normal);
        }
        Some(output)
    }

    /// Given a transform, compute the associated axis aligned bounding box for a child shape.
	/// @param xf the world transform of the shape.
    fn compute_aabb(&self, xf: Transform, _: i32) -> AABB {
        let v1 = mul_transform_vec2(xf, self.m_vertex1);
        let v2 = mul_transform_vec2(xf, self.m_vertex2);

        let lower = min_vec(v1, v2);
        let upper = max_vec(v1, v2);

        let r = Vec2::new(self.m_radius, self.m_radius);
        let mut aabb = AABB::new();
        aabb.lower_bound = lower - r;
        aabb.upper_bound = upper + r;
        return aabb;
    }

    /// Compute the mass properties of this shape using its dimensions and density.
	/// The inertia tensor is computed about the local origin.
    fn compute_mass(&self, _: f32) -> MassData {
        let mut mass_data = MassData::new();
        mass_data.mass = 0.0;
        mass_data.center = 0.5 * (self.m_vertex1 + self.m_vertex2);
        mass_data.I = 0.0;
        return mass_data;
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

impl ToDerivedShape for EdgeShape {
    fn circle(&self) -> Option<CircleShape> {
        None
    }

    fn edge(&self) -> Option<EdgeShape> {
        Some(*self)
    }

    fn polygon(&self) -> Option<PolygonShape> {
        None
    }

    fn chain(&self) -> Option<ChainShape> {
        None
    }
}
