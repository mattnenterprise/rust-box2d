use super::super::body::Body;
use super::super::math::Vec2;
use super::super::manifold::Manifold;
use super::super::shape::shape::Shape::CircleShape;
use super::collider_result::ColliderResult;
use super::collider::Collider;

pub struct CircleCircleCollider {
    pair: (Body, Body)
}

impl Collider for CircleCircleCollider {
    fn new(pair: (Body, Body)) -> CircleCircleCollider {
        return CircleCircleCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let a_shape = self.pair().0.shape;
        let b_shape = self.pair().1.shape;

        match (a_shape, b_shape) {
            (CircleShape{center: center_a, radius: radius_a}, CircleShape{center: center_b, radius: radius_b}) => {
                let normal = center_a - center_b;
                let mut total_radius = radius_a + radius_b;
                total_radius *= total_radius;

                if normal.length() * normal.length() > total_radius {
                    return ColliderResult::new_empty_false();
                }

                let distance = normal.length();

                let mut manifold = Manifold{body_a: self.pair().0, body_b: self.pair().1, normal: Vec2::new(0.0, 0.0), penetration: 0.0};

                if distance != 0.0 {
                    manifold.penetration = f32::sqrt(total_radius) - distance;
                    manifold.normal = normal.normal();
                } else {
                    manifold.penetration = radius_a;
                    manifold.normal = Vec2::new(1.0, 0.0);
                }

                ColliderResult::new(Some(manifold), true)
            }
            _ => {
                panic!("Something happened. Cannot test circle to circel collision without circles!!!");
            }
        }
    }
}
