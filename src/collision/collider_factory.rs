use super::super::shape::shape::Shape::{CircleShape, LineShape, ChainLineShape, PolygonShape};
use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;
use super::circle_circle_collider::CircleCircleCollider;
use super::circle_line_collider::CircleLineCollider;
use super::circle_polygon_collider::CirclePolygonCollider;
use super::circle_chain_line_collider::CircleChainLineCollider;
use super::polygon_chain_line_collider::PolygonChainLineCollider;
use super::polygon_line_collider::PolygonLineCollider;
use super::polygon_polygon_collider::PolygonPolygonCollider;

pub fn collider_factory(body_pair: (Body, Body)) -> ColliderResult {
    let a_body = body_pair.0;
    let b_body = body_pair.1;
    let ref a_shape = a_body.shape;
    let ref b_shape = b_body.shape;

    match (a_shape, b_shape) {
        (&CircleShape{..}, &CircleShape{..}) => {
            CircleCircleCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&CircleShape{..}, &LineShape{..}) => {
            CircleLineCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&LineShape{..}, &CircleShape{..}) => {
            CircleLineCollider::new((b_body.clone(), a_body.clone())).colliding()
        },
        (&CircleShape{..}, &ChainLineShape{..}) => {
            CircleChainLineCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&ChainLineShape{..}, &CircleShape{..}) => {
            CircleChainLineCollider::new((b_body.clone(), a_body.clone())).colliding()
        },
        (&CircleShape{..}, &PolygonShape{..}) => {
            CirclePolygonCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&PolygonShape{..}, &CircleShape{..}) => {
            CirclePolygonCollider::new((b_body.clone(), a_body.clone())).colliding()
        },
        (&PolygonShape{..}, &ChainLineShape{..}) => {
            PolygonChainLineCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&ChainLineShape{..}, &PolygonShape{..}) => {
            PolygonChainLineCollider::new((b_body.clone(), a_body.clone())).colliding()
        },
        (&PolygonShape{..}, &PolygonShape{..}) => {
            PolygonPolygonCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&PolygonShape{..}, &LineShape{..}) => {
            PolygonLineCollider::new((a_body.clone(), b_body.clone())).colliding()
        },
        (&LineShape{..}, &PolygonShape{..}) => {
            PolygonLineCollider::new((b_body.clone(), a_body.clone())).colliding()
        },
        _ => {
            ColliderResult::new_empty_false()
        }
    }
}
