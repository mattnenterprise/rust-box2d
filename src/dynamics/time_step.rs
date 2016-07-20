/// This is an internal structure.
pub struct TimeStep {
    // time step
    pub dt: f32,
    // inverse time step (0 if dt == 0).
    pub inv_dt: f32,
    // dt * inv_dt0
    pub dt_ratio: f32,
    pub velocity_iterations: i32,
    pub position_interations: i32,
    pub warm_starting: bool
}

impl TimeStep {
    pub fn new() -> TimeStep {
        TimeStep {
            dt: 0.0,
            inv_dt: 0.0,
            dt_ratio: 0.0,
            velocity_iterations: 0,
            position_interations: 0,
            warm_starting: false // TODO IDK about this ?
        }
    }
}
