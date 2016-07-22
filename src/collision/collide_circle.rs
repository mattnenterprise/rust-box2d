use super::manifold::{Manifold, ManifoldType};
use super::shapes::circle_shape::CircleShape;
use super::contact_id::ContactID;
use super::super::common::Transform;
use super::super::common::Vec2;
use super::super::common::math::mul_transform_vec2;

pub fn collide_circles(circle_a: CircleShape, xf_a: Transform, circle_b: CircleShape, xf_b: Transform) -> Manifold {
    let mut manifold = Manifold::new();
    manifold.point_count = 0;

    let p_a = mul_transform_vec2(xf_a, circle_a.m_p);
    let p_b = mul_transform_vec2(xf_b, circle_b.m_p);

    let d = p_b - p_a;
    let dist_sqr = Vec2::dot(d, d);
    let r_a = circle_a.m_radius;
    let r_b = circle_b.m_radius;
    let radius = r_a + r_b;
    if dist_sqr > radius * radius {
        // TODO Return manifold or make return an Option ?
        return manifold;
    }

    manifold.typ = ManifoldType::Circles;
    manifold.local_point = circle_b.m_p;
    manifold.local_normal.set_zero();
    manifold.point_count = 1;

    manifold.points[0].local_point = circle_b.m_p;
    manifold.points[0].id = ContactID::new();

    return manifold;
}
