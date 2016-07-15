use super::super::shape::shape::Shape::{CircleShape, ChainLineShape};
use super::super::body::Body;
use super::super::math::Vec2;
use super::super::manifold::Manifold;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CircleChainLineCollider {
    pair: (Body, Body)
}

impl Collider for CircleChainLineCollider {
    fn new(pair: (Body, Body)) -> CircleChainLineCollider {
        return CircleChainLineCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let circle_shape = self.pair().0.shape;
        let chain_line_shape = self.pair().1.shape;

        match (circle_shape, chain_line_shape) {
            (CircleShape{center, radius}, ChainLineShape{points}) => {
                let global_circle_center = self.pair().0.position + center;
                let mut closest_line_distance = Vec2::new(0.0, 0.0);

                for i in 0..(points.len()-1) {
                    let global_point1 = points[i] + self.pair().1.position;
                    let global_point2 = points[i+1] + self.pair().1.position;

                    let segment_vector = global_point1 - global_point2;
                    let mut point_vector = global_circle_center - global_point2;

                    let scalar_projection = point_vector.dot(segment_vector.normal());

                    let mut closest_point = global_point2 + segment_vector.normal() * scalar_projection;

                    if scalar_projection < 0.0 {
                        closest_point = global_point2;
                    } else if scalar_projection > segment_vector.length() {
                        closest_point = global_point1;
                    }

                    let distance_vector = global_circle_center - closest_point;

                    if i == 0 || distance_vector.length() < closest_line_distance.length() {
                        closest_line_distance = distance_vector;
                    }
                }

                if closest_line_distance.length() < radius {
                    let offset = closest_line_distance.normal() * (radius - closest_line_distance.length());
                    let mut manifold = Manifold{body_a: self.pair().0, body_b: self.pair().1, normal: Vec2::new(0.0, 0.0), penetration: 0.0};
                    manifold.normal = offset.normal();
                    manifold.penetration = offset.length();
                    return ColliderResult::new(Some(manifold), true);
                }

                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test circle to chain line collision without circle and chain line!!!");
            }
        }
    }
}
