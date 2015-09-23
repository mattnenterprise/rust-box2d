use super::super::shape::shape::Shape::{CircleShape, LineShape};
use super::super::body::Body;
use super::super::math::Vec2;
use super::super::manifold::Manifold;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CircleLineCollider {
    pair: (Body, Body)
}

impl Collider for CircleLineCollider {
    fn new(pair: (Body, Body)) -> CircleLineCollider {
        return CircleLineCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let circle_shape = self.pair().0.shape;
        let line_shape = self.pair().1.shape;

        match (circle_shape, line_shape) {
            (CircleShape{center, radius}, LineShape{point1, point2}) => {
                let global_circle_center = self.pair().0.position + center;
                let global_point1 = self.pair().1.position + point1;
                let global_point2 = self.pair().1.position + point2;

                let segment_vector = global_point1 - global_point2;
                let mut point_vector = global_circle_center - global_point2;

                let scalar_projection = point_vector.dot(segment_vector.normal());

                let mut closest_point = global_point2 + (segment_vector.normal().multiply(scalar_projection));

                if scalar_projection < 0.0 {
                    closest_point = global_point2;
                } else if scalar_projection > segment_vector.length() {
                    closest_point = global_point1;
                }

                let distance_vector = global_circle_center - closest_point;

                if distance_vector.length() < radius {
                    let offset = distance_vector.normal().multiply((radius - distance_vector.length()));
                    let mut manifold = Manifold{body_a: self.pair().0, body_b: self.pair().1, normal: Vec2::new(0.0, 0.0), penetration: 0.0};
                    manifold.normal = offset.normal();
                    manifold.penetration = offset.length();
                    return ColliderResult::new(Some(manifold), true);
                }
                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test circle to line collision without circle and line!!!");
            }
        }
    }
}
