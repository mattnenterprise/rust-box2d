use super::body::Body;
use super::collision::collider_result::ColliderResult;
use super::narrow_phase::NarrowPhase;
use super::collision::collider_factory::collider_factory;

pub struct DefaultNarrowPhase;

impl NarrowPhase for DefaultNarrowPhase {
    fn run(&self, bodies: &Vec<(Body, Body)>) -> Vec<ColliderResult> {
        let mut c_results: Vec<ColliderResult> = Vec::new();
        for pair in bodies.iter() {
            c_results.push(collider_factory(pair.clone()));
        }
        return c_results;
    }
}

impl DefaultNarrowPhase {
    pub fn new() -> DefaultNarrowPhase {
        return DefaultNarrowPhase
    }
}
