use super::super::shape::shape::Shape::{CircleShape, PolygonShape};
use super::super::body::Body;
use super::super::common::Vec2;
use super::super::manifold::Manifold;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CirclePolygonCollider {
    pair: (Body, Body)
}
 //TODO Check to make sur this fully works
impl Collider for CirclePolygonCollider {
    fn new(pair: (Body, Body)) -> CirclePolygonCollider {
        return CirclePolygonCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let circle_shape = self.pair().0.shape;
        let polygon_shape = self.pair().1.shape;

        match (circle_shape, polygon_shape) {
            (CircleShape{center, radius}, PolygonShape{points}) => {

                let global_circle_center = center + self.pair().0.position;
                let mut closest_line_distance = Vec2::new(0.0, 0.0);

                for index in 0..(points.len() -1) {
                    let global_p1: Vec2 = points[index] + self.pair().1.position;
                    let global_p2: Vec2 = points[index + 1] + self.pair().1.position;

                    let segment_vector = global_p1 - global_p2;
                    let point_vector = global_circle_center - global_p2;

                    let scalar_projection: f32 = Vec2::dot(point_vector, segment_vector.normal());

                    let mut closest_point = Vec2::new(0.0, 0.0);

                    if scalar_projection < 0.0 {
                        closest_point = global_p2;
                    } else if scalar_projection > segment_vector.length() {
                        closest_point = global_p1;
                    } else {
                        closest_point = global_p2 + segment_vector.normal() * scalar_projection;
                    }

                    let distance_vector = global_circle_center - closest_point;

                    if index == 0 {
                        closest_line_distance = distance_vector;
                    } else {
                        if distance_vector.length() < closest_line_distance.length() {
                            closest_line_distance = distance_vector;
                        }
                    }
                }

                let global_p1: Vec2 = points[points.len()-1] + self.pair().1.position;
                let global_p2: Vec2 = points[0] + self.pair().1.position;
                let segment_vector = global_p1 - global_p2;
                let point_vector = global_circle_center - global_p2;

                let scalar_projection: f32 = Vec2::dot(point_vector, segment_vector.normal());

                let mut closest_point = Vec2::new(0.0, 0.0);

                if scalar_projection < 0.0 {
                    closest_point = global_p2;
                } else if scalar_projection > segment_vector.length() {
                    closest_point = global_p1;
                } else {
                    closest_point = global_p2 + segment_vector.normal() * scalar_projection;
                }

                let distance_vector = global_circle_center - closest_point;

                if distance_vector.length() < closest_line_distance.length() {
                    closest_line_distance = distance_vector;
                }

                if closest_line_distance.length() < radius {
                    let offset = closest_line_distance.normal() * (radius - closest_line_distance.length());
                    let manifold = Manifold{body_a: self.pair().0, body_b: self.pair().1, normal: offset.normal(), penetration: offset.length()};
                    return ColliderResult::new(Some(manifold), true);
                }

                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test circle to polygon collision without circle and polygon!!!");
            }
        }
    }
}
