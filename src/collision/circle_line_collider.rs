use super::super::body_pair::BodyPair;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CircleLineCollider {
    pair: BodyPair
}

impl Collider for CircleLineCollider {
    fn new(pair: BodyPair) -> CircleLineCollider {
        return CircleLineCollider{ pair: pair }
    }

    fn pair(&self) -> BodyPair {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        return ColliderResult;
    }
}
