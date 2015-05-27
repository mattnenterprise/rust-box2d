use super::super::body_pair::BodyPair;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CircleCircleCollider {
    pair: BodyPair
}

impl Collider for CircleCircleCollider {
    fn new(pair: BodyPair) -> CircleCircleCollider {
        return CircleCircleCollider{ pair: pair }
    }

    fn pair(&self) -> BodyPair {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let a = self.pair().body_a;
        let b = self.pair().body_b;
        return ColliderResult;
    }
}
