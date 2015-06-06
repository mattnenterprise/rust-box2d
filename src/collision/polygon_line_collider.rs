use super::super::shape::shape::Shape::{PolygonShape, LineShape};
use super::super::body::Body;
use super::super::math::Vec2;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct PolygonLineCollider {
    pair: (Body, Body)
}

impl Collider for PolygonLineCollider {
    fn new(pair: (Body, Body)) -> PolygonLineCollider {
        return PolygonLineCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let polygon_shape = self.pair().0.shape;
        let line_shape = self.pair().1.shape;

        match (polygon_shape, line_shape) {
            (PolygonShape{points}, LineShape{point1, point2}) => {
                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test polygon to chain line collision without polygon and chain line!!!");
            }
        }
    }
}
