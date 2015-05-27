use super::body::Body;
use super::manifold::Manifold;
use super::broad_phase::BroadPhase;

pub struct DefaultBroadPhase;

impl BroadPhase for DefaultBroadPhase {
    fn run(&self, bodies: &Vec<Body>) -> Vec<Manifold> {
        // The worst implementation possible!!!
        let mut manifolds = Vec::new();
        for (i, body1) in bodies.iter().enumerate() {
            for (j, body2) in bodies.iter().enumerate() {
                if i != j {
                    manifolds.push(Manifold{body_a: (*body1).clone(), body_b: (*body2).clone()});
                }
            }
        }
        return manifolds;
    }
}

impl DefaultBroadPhase {
    pub fn new() -> DefaultBroadPhase {
        return DefaultBroadPhase
    }
}
