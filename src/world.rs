use super::math::Vec2;
use super::body::Body;
use super::broad_phase::BroadPhase;
use super::default_broad_phase::DefaultBroadPhase;

pub struct World {
	gravity: Vec2,
	broad_phase: Box<BroadPhase>,
	bodies: Vec<Body>
}

impl World {
	pub fn new(gravity: Vec2) -> World {
		return World{ gravity: gravity, broad_phase: Box::new(DefaultBroadPhase::new()) , bodies: Vec::new() }
	}

	pub fn add_body(&mut self, body: Body) {
		self.bodies.push(body);
	}

	pub fn clear(&mut self) {
		self.bodies.clear();
	}

	pub fn step(&mut self, timeStep: f32) {
		let pairs = self.broad_phase.run(&self.bodies);

	}
}
