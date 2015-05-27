use super::super::body_pair::BodyPair;
use super::collider_result::ColliderResult;

pub trait Collider {
    fn new(pair: BodyPair) -> Self;
    fn pair(&self) -> BodyPair;
    fn colliding(&self) -> ColliderResult;
}
