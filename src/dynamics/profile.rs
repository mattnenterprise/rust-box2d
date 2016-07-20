/// Profiling data. Times are in milliseconds.
pub struct Profile {
    pub step: f32,
	pub collide: f32,
	pub solve: f32,
	pub solve_init: f32,
	pub solve_velocity: f32,
	pub solve_position: f32,
	pub broadphase: f32,
	pub solve_toi: f32
}

impl Profile {
    pub fn new() -> Profile {
        Profile {
            step: 0.0,
        	collide: 0.0,
        	solve: 0.0,
        	solve_init: 0.0,
        	solve_velocity: 0.0,
        	solve_position: 0.0,
        	broadphase: 0.0,
        	solve_toi: 0.0
        }
    }
}
