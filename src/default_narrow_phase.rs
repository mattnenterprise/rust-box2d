use super::body::Body;
use super::collision::collider_result::ColliderResult;
use super::narrow_phase::NarrowPhase;

pub struct DefaultNarrowPhase;

impl NarrowPhase for DefaultNarrowPhase {
    fn run(&self, bodies: &Vec<(Body, Body)>) -> Vec<ColliderResult> {
        let mut c_results: Vec<ColliderResult> = Vec::new();
        return c_results;
    }
}

impl DefaultNarrowPhase {
    pub fn new() -> DefaultNarrowPhase {
        return DefaultNarrowPhase
    }
}
