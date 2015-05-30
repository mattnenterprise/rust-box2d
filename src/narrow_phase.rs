use super::body::Body;
use super::collision::collider_result::ColliderResult;

pub trait NarrowPhase {
    fn run(&self, results: &Vec<(Body, Body)>) -> Vec<ColliderResult>;
}
