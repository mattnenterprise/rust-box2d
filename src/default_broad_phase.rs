use super::body::Body;
use super::broad_phase::BroadPhase;

pub struct DefaultBroadPhase;

impl BroadPhase for DefaultBroadPhase {
    fn run(&self, bodies: &Vec<Body>) -> Vec<(Body, Body)> {
        // The worst implementation possible!!!
        let mut pairs: Vec<(Body, Body)> = Vec::new();
        for (i, body1) in bodies.iter().enumerate() {
            for (j, body2) in bodies.iter().enumerate() {
                if i != j {
                    pairs.push( ((*body1).clone(), (*body2).clone()) );
                }
            }
        }
        return pairs;
    }
}

impl DefaultBroadPhase {
    pub fn new() -> DefaultBroadPhase {
        return DefaultBroadPhase
    }
}
