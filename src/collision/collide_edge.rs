use super::manifold::{Manifold, ManifoldType};
use super::shapes::circle_shape::CircleShape;
use super::shapes::edge_shape::EdgeShape;
use super::super::common::Transform;

pub fn collide_edge_and_circle(edge_a: EdgeShape, xf_a: Transform, circle_b: CircleShape, xf_b: Transform) -> Manifold {
    let manifold = Manifold::new();
    return manifold;
}
