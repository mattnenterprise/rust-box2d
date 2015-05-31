use super::super::shape::shape::Shape::{CircleShape, LineShape};
use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;
use super::circle_circle_collider::CircleCircleCollider;
use super::circle_line_collider::CircleLineCollider;

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
        }
        _ => {
            ColliderResult::new_empty_false()
        }
    }
}
