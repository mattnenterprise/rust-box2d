use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;
use super::super::shape::shape::Shape::CircleShape;

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

                if(normal.length() * normal.length() > total_radius) {
                    //No Collision
                }

                let distance = normal.length();



            }
            _ => {
                //Something is wrong
            }
        };

        return ColliderResult;
    }
}
