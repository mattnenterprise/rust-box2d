use super::manifold::Manifold;

pub trait CollisionResolution {
    fn run(&self, manifold: &Vec<Manifold>);
}
