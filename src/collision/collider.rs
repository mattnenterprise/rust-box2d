use super::super::body::Body;
use super::collider_result::ColliderResult;

pub trait Collider {
    fn new(pair: (Body, Body)) -> Self;
    fn pair(&self) -> (Body, Body);
    fn colliding(&self) -> ColliderResult;
}
