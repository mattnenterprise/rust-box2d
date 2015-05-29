use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CircleLineCollider {
    pair: (Body, Body)
}

impl Collider for CircleLineCollider {
    fn new(pair: (Body, Body)) -> CircleLineCollider {
        return CircleLineCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        return ColliderResult;
    }
}
