use super::body::Body;
use super::manifold::Manifold;

pub trait BroadPhase {
    fn run(&self, bodies: &Vec<Body>) -> Vec<Manifold>;
}
