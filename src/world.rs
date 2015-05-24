use super::math::Vec2;
use super::body::Body;
use super::shape::shape::Shape;

struct World {
	gravity: Vec2,
	bodies: Vec<Body>
}

impl World {
	pub fn new(gravity: Vec2) -> World {
		return World{ gravity: gravity, bodies: Vec::new() }
	}

	pub fn add_body(&mut self, body: Body) {
		self.bodies.push(body);
	}

	pub fn clear(&mut self) {
		self.bodies.clear();
	}

	pub fn step(timeStep: f32) {
	}
}
