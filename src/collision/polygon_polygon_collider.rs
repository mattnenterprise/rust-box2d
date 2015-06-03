use super::super::shape::shape::Shape::PolygonShape;
use super::super::body::Body;
use super::super::math::Vec2;
use super::super::manifold::Manifold;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct PolygonPolygonCollider {
    pair: (Body, Body)
}

impl Collider for PolygonPolygonCollider {
    fn new(pair: (Body, Body)) -> PolygonPolygonCollider {
        return PolygonPolygonCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let polygon_a_shape = self.pair().0.shape;
        let polygon_b_shape = self.pair().1.shape;

        match (polygon_a_shape, polygon_b_shape) {
            (PolygonShape{points: points_a}, PolygonShape{points: points_b}) => {

                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test polygon to polygon collision without two polygons!!!");
            }
        }
    }
}
