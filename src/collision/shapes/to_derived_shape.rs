use super::circle_shape::CircleShape;
use super::edge_shape::EdgeShape;
use super::polygon_shape::PolygonShape;
use super::chain_shape::ChainShape;

pub trait ToDerivedShape {
    fn circle(&self) -> Option<CircleShape>;
    fn edge(&self) -> Option<EdgeShape>;
    fn polygon(&self) -> Option<PolygonShape>;
    fn chain(&self) -> Option<ChainShape>;
}
