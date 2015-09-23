use super::super::shape::shape::Shape::{PolygonShape, ChainLineShape};
use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct PolygonChainLineCollider {
    pair: (Body, Body)
}

impl Collider for PolygonChainLineCollider {
    fn new(pair: (Body, Body)) -> PolygonChainLineCollider {
        return PolygonChainLineCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let polygon_shape = self.pair().0.shape;
        let chain_line_shape = self.pair().1.shape;

        match (polygon_shape, chain_line_shape) {
            (PolygonShape{..}, ChainLineShape{..}) => {
                return ColliderResult::new_empty_false();
            },
            _ => {
                panic!("Something happened. Cannot test polygon to chain line collision without polygon and chain line!!!");
            }
        }
    }
}
