use super::collision_resolution::CollisionResolution;
use super::manifold::Manifold;

pub struct DefaultCollisionResolution;

impl CollisionResolution for DefaultCollisionResolution {
    fn run(&self, manifolds: &Vec<Manifold>) {

    }
}

impl DefaultCollisionResolution {
    pub fn new() -> DefaultCollisionResolution {
        return DefaultCollisionResolution
    }
}
