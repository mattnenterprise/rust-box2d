use super::manifold::{Manifold, ManifoldType};
use super::shapes::circle_shape::CircleShape;
use super::shapes::polygon_shape::PolygonShape;
use super::contact_id::ContactID;
use super::super::common::Transform;
use super::super::common::Vec2;
use super::super::common::math::{mul_transform_vec2, mul_transform_vec2_inverse, vec_distance_squared};
use std::f32;

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

pub fn collide_polygon_and_circle(polygon_a: PolygonShape, xf_a: Transform, circle_b: CircleShape, xf_b: Transform) -> Manifold {
    let mut manifold = Manifold::new();
    manifold.point_count = 0;

    // Compute circle position in the frame of the polygon.
    let c = mul_transform_vec2(xf_b, circle_b.m_p);
    let c_local = mul_transform_vec2_inverse(xf_a, c);

    // Find the min separating edge.
	let mut normal_index = 0;
	let mut separation = -f32::MAX;
	let radius = polygon_a.m_radius + circle_b.m_radius;
	let vertex_count = polygon_a.m_count;
	let vertices = polygon_a.m_vertices;
	let normals = polygon_a.m_normals;

    for i in 0..vertex_count {
        let s = Vec2::dot(normals[i as usize], c_local - vertices[i as usize]);
        if s > radius {
            // TODO Return manifold or make return an Option ?
            return manifold;
        }

        if s > separation {
            separation = s;
            normal_index = i;
        }
    }

    // Vertices that subtend the incident face.
	let vert_index1 = normal_index;
	let vert_index2 = if vert_index1 + 1 < vertex_count {
         vert_index1 + 1
        } else {
            0
        };
	let v1 = vertices[vert_index1 as usize];
	let v2 = vertices[vert_index2 as usize];

    if separation < f32::EPSILON {
        manifold.point_count = 1;
        manifold.typ = ManifoldType::FaceA;
        manifold.local_normal = normals[normal_index as usize];
        manifold.local_point = 0.5 * (v1 + v2);
        manifold.points[0].local_point = circle_b.m_p;
        manifold.points[0].id = ContactID::new();
        return manifold;
    }

    // Compute barycentric coordinates
	let u1 = Vec2::dot(c_local - v1, v2 - v1);
    let u2 = Vec2::dot(c_local - v2, v1 - v2);
    if u1 <= 0.0 {
        if vec_distance_squared(c_local, v1) > radius * radius {
            // TODO Return manifold or make return an Option ?
            return manifold;
        }

        manifold.point_count = 1;
        manifold.typ = ManifoldType::FaceA;
        manifold.local_normal = c_local - v1;
        manifold.local_normal.normalize();
        manifold.local_point = v1;
        manifold.points[0].local_point = circle_b.m_p;
        manifold.points[0].id = ContactID::new();

    } else if u2 <= 0.0 {
        if vec_distance_squared(c_local, v2) > radius * radius {
            // TODO Return manifold or make return an Option ?
            return manifold;
        }
        manifold.point_count = 1;
        manifold.typ = ManifoldType::FaceA;
        manifold.local_normal = c_local - v2;
        manifold.local_normal.normalize();
        manifold.local_point = v2;
        manifold.points[0].local_point = circle_b.m_p;
        manifold.points[0].id = ContactID::new();
    } else {
        let face_center = 0.5 * (v1 + v2);
        let s = Vec2::dot(c_local - face_center, normals[vert_index1 as usize]);
		if s > radius {
            // TODO Return manifold or make return an Option ?
			return manifold;
		}

		manifold.point_count = 1;
		manifold.typ = ManifoldType::FaceA;
		manifold.local_normal = normals[vert_index1 as usize];
		manifold.local_point = face_center;
		manifold.points[0].local_point = circle_b.m_p;
		manifold.points[0].id = ContactID::new();;
    }

    return manifold;
}
