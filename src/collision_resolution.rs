use super::manifold::Manifold;

pub trait CollisionResolution {
    fn resolve_collisions(&mut self, manifold: &Vec<Manifold>);
}
