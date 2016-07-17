use super::super::super::common::Vec2;
use super::super::super::common::Transform;
use super::super::super::common::math::mul_rot_vec2;
use super::super::ray_cast_input::RayCastInput;
use super::super::ray_cast_output::RayCastOutput;
use super::super::aabb::AABB;
use super::shape::{Shape, ShapeType};
use super::mass_data::MassData;
use std::f32::consts::PI;
use std::f32::EPSILON;

/// A circle shape.
pub struct CircleShape {
    pub m_p: Vec2,
    pub m_radius: f32
}

impl CircleShape {
    pub fn new() -> CircleShape {
        CircleShape {
            m_p: Vec2::zero(),
            m_radius: 0.0
        }
    }
}

impl Shape for CircleShape {
    /// Get the type of this shape. You can use this to down cast to the concrete shape.
    /// @return the shape type.
    fn get_type(&self) -> ShapeType {
        ShapeType::Circle
    }

    /// Get the number of child primitives.
    fn get_child_count(&self) -> i32 {
        1
    }

    /// Test a point for containment in this shape. This only works for convex shapes.
    /// @param xf the shape world transform.
    /// @param p a point in world coordinates.
    fn test_point(&self, transform: Transform, p: Vec2) -> bool {
        let center = transform.p + mul_rot_vec2(transform.q, self.m_p);
        let d = p - center;
        Vec2::dot(d, d) <= self.m_radius * self.m_radius
    }

    /// Cast a ray against a child shape.
	/// @param output the ray-cast results.
	/// @param input the ray-cast input parameters.
	/// @param transform the transform to be applied to the shape.
	/// @param child_index the child shape index
    fn ray_cast(&self, input: RayCastInput, transform: Transform, _: i32) -> Option<RayCastOutput> {
        let position = transform.p + mul_rot_vec2(transform.q, self.m_p);
        let s = input.p1 - position;
        let b = Vec2::dot(s, s) - self.m_radius * self.m_radius;

        // Solve quadratic equation.
        let r = input.p2 - input.p1;
        let c = Vec2::dot(s, r);
        let rr = Vec2::dot(r, r);
        let sigma = c * c - rr * b;
        if sigma < 0.0 || rr < EPSILON {
            return None
        }

        let mut a = -(c + f32::sqrt(sigma));

        if 0.0 <= a && a <= input.max_fraction * rr {
            a /= rr;
            let mut output = RayCastOutput::new();
            output.fraction = a;
            output.normal = s + a * r;
            output.normal.normalize();
            return Some(output);
        }

        return None;
    }

    /// Given a transform, compute the associated axis aligned bounding box for a child shape.
	/// @param xf the world transform of the shape.
	/// @param child_index the child shape.
    fn compute_aabb(&self, transform: Transform, _: i32) -> AABB {
        let p = transform.p + mul_rot_vec2(transform.q, self.m_p);
        let mut aabb = AABB::new();
        aabb.lower_bound.set(p.x - self.m_radius, p.y - self.m_radius);
        aabb.upper_bound.set(p.x + self.m_radius, p.y + self.m_radius);
        return aabb;
    }

    /// Compute the mass properties of this shape using its dimensions and density.
	/// The inertia tensor is computed about the local origin.
	/// @param density the density in kilograms per meter squared.
    fn compute_mass(&self, density: f32) -> MassData {
        let mut mass_data = MassData::new();
        mass_data.mass = density * PI * self.m_radius * self.m_radius;
        mass_data.center = self.m_p;
        mass_data.I = mass_data.mass * (0.5 * self.m_radius * self.m_radius + Vec2::dot(self.m_p, self.m_p));
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
