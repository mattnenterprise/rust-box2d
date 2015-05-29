use super::super::body::Body;
use super::collider::Collider;
use super::collider_result::ColliderResult;

pub struct CircleCircleCollider {
    pair: (Body, Body)
}

impl Collider for CircleCircleCollider {
    fn new(pair: (Body, Body)) -> CircleCircleCollider {
        return CircleCircleCollider{ pair: pair }
    }

    fn pair(&self) -> (Body, Body) {
        return self.pair.clone();
    }

    fn colliding(&self) -> ColliderResult {
        let a = self.pair().0;
        let b = self.pair().1;
        return ColliderResult;
    }
}
