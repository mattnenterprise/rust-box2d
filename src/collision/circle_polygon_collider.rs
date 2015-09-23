use super::super::shape::shape::Shape::{CircleShape, PolygonShape};
use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CirclePolygonCollider {
    pair: (Body, Body)
}

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
            (CircleShape{..}, PolygonShape{..}) => {
                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test circle to polygon collision without circle and polygon!!!");
            }
        }
    }
}
