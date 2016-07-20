use super::super::common::Vec2;
use super::time_step::TimeStep;

pub struct SolverData {
    pub step: TimeStep,
    pub positions: Vec<Vec2>,
    pub velocities: Vec<Vec2>
}

impl SolverData {
    pub fn new() -> SolverData {
        SolverData {
            step: TimeStep::new(),
            positions: Vec::new(),
            velocities: Vec::new()
        }
    }
}
