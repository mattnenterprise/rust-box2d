use super::super::shape::shape::Shape::{CircleShape, LineShape};
use super::super::body::Body;
use super::collider::Collider;

fn collider_factory(body_pair: (Body, Body)) {
    let a = body_pair.0.shape;
    let b = body_pair.1.shape;

    match (a, b) {
        (CircleShape{center, radius}, LineShape{point1, point2}) => {
            //Do important stuff here
        },
        _ => {
            //TODO actually do something here
        }
    }
}
