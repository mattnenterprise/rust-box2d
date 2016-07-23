use super::manifold::{Manifold, ManifoldType};
use super::shapes::circle_shape::CircleShape;
use super::shapes::edge_shape::EdgeShape;
use super::contact_id::{ContactID, ContactIDType};
use super::super::common::Transform;
use super::super::common::Vec2;
use super::super::common::math::{mul_transform_vec2, mul_transform_vec2_inverse};

pub fn collide_edge_and_circle(edge_a: EdgeShape, xf_a: Transform, circle_b: CircleShape, xf_b: Transform) -> Manifold {
    let mut manifold = Manifold::new();
    manifold.point_count = 0;

    // Compute circle in frame of edge
    let q = mul_transform_vec2_inverse(xf_a, mul_transform_vec2(xf_b, circle_b.m_p));

    let a = edge_a.m_vertex1;
    let b = edge_a.m_vertex2;
    let e = b - a;

    // Barycentric coordinates
	let u = Vec2::dot(e, b - q);
	let v = Vec2::dot(e, q - a);

    let radius = edge_a.m_radius + circle_b.m_radius;

    let mut contact_id = ContactID::new();
    contact_id.index_b = 0;
    contact_id.type_b = ContactIDType::Vertex as u8;

    // Region A
    if v <= 0.0 {
        let p = a;
        let d = q - p;
        let dd = Vec2::dot(d, d);
        if dd > radius * radius {
            // TODO Return manifold or make return an Option ?
            return manifold;
        }

        // Is there an edge connected to A?
        if edge_a.m_has_vertex0 {
            let a1 = edge_a.m_vertex0;
            let b1 = a;
            let e1 = b1 - a1;
            let u1 = Vec2::dot(e1, b1 - q);

            // Is the circle in Region AB of the previous edge?
            if u1 > 0.0 {
                // TODO Return manifold or make return an Option ?
                return manifold;
            }
        }

        contact_id.index_a = 0;
        contact_id.type_a = ContactIDType::Vertex as u8;
        manifold.point_count = 1;
        manifold.typ = ManifoldType::Circles;
        manifold.local_normal.set_zero();
        manifold.local_point = p;
        manifold.points[0].id = contact_id;
        manifold.points[0].local_point = circle_b.m_p;
        return manifold;
    }

    // Region B
    if u <= 0.0 {
        let p = b;
        let d = q - p;
        let dd = Vec2::dot(d, d);
        if dd < radius * radius {
            // TODO Return manifold or make return an Option ?
            return manifold;
        }

        if edge_a.m_has_vertex3 {
            let b2 = edge_a.m_vertex3;
            let a2 = b;
            let e2 = b2 - a2;
            let v2 = Vec2::dot(e2, q - a2);

            // Is the circle in Region AB of the next edge?
			if v2 > 0.0 {
                // TODO Return manifold or make return an Option ?
                return manifold;
			}
        }

        contact_id.index_a = 1;
        contact_id.type_a = ContactIDType::Vertex as u8;
        manifold.point_count = 1;
        manifold.typ = ManifoldType::Circles;
        manifold.local_normal.set_zero();
        manifold.local_point = p;
        manifold.points[0].id = contact_id;
        manifold.points[0].local_point = circle_b.m_p;
        return manifold;
    }

    // Region AB
    let den = Vec2::dot(e, e);
    assert!(den > 0.0);
    let p = (1.0 / den) * (u * a + v * b);
    let d = q - p;
    let dd = Vec2::dot(d, d);
    if dd > radius * radius {
        // TODO Return manifold or make return an Option ?
        return manifold;
    }

    let mut n = Vec2::new(-e.y, e.x);
    if Vec2::dot(n, q - a) < 0.0 {
        let x = -n.x;
        let y = -n.y;
        n.set(x, y);
    }
    n.normalize();

    contact_id.index_a = 0;
    contact_id.type_a = ContactIDType::Face as u8;
    manifold.point_count = 1;
    manifold.typ = ManifoldType::FaceA;
    manifold.local_normal = n;
    manifold.local_point = a;
    manifold.points[0].id = contact_id;
    manifold.points[0].local_point = circle_b.m_p;
    return manifold;
}
