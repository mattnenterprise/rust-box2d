use super::body::Body;

pub trait BroadPhase {
    fn run(&self, bodies: &Vec<Body>) -> Vec<(Body, Body)>;
}
