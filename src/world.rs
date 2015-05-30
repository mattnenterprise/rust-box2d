use super::math::Vec2;
use super::body::Body;
use super::broad_phase::BroadPhase;
use super::default_broad_phase::DefaultBroadPhase;
use super::narrow_phase::NarrowPhase;
use super::default_narrow_phase::DefaultNarrowPhase;

pub struct World {
	gravity: Vec2,
	broad_phase: Box<BroadPhase>,
	narrow_phase: Box<NarrowPhase>,
	bodies: Vec<Body>
}

impl World {
	pub fn new(gravity: Vec2) -> World {
		return World{ gravity: gravity, broad_phase: Box::new(DefaultBroadPhase::new()), narrow_phase: Box::new(DefaultNarrowPhase::new()), bodies: Vec::new() }
	}

	pub fn add_body(&mut self, body: Body) {
		self.bodies.push(body);
	}

	pub fn clear(&mut self) {
		self.bodies.clear();
	}

	pub fn step(&mut self, timeStep: f32) {
		let pairs = self.broad_phase.run(&self.bodies);
		let collider_results = self.narrow_phase.run(&pairs);


	}
}
