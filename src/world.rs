use super::math::Vec2;
use super::body::Body;
use super::manifold::Manifold;
use super::broad_phase::BroadPhase;
use super::default_broad_phase::DefaultBroadPhase;
use super::narrow_phase::NarrowPhase;
use super::default_narrow_phase::DefaultNarrowPhase;
use super::collision_resolution::CollisionResolution;
use super::default_collision_resolution::DefaultCollisionResolution;

pub struct World {
	gravity: Vec2,
	broad_phase: Box<BroadPhase>,
	narrow_phase: Box<NarrowPhase>,
	collision_resolution: Box<CollisionResolution>,
	bodies: Vec<Body>
}

impl World {
	pub fn new(gravity: Vec2) -> World {
		return World{ gravity: gravity,
					broad_phase: Box::new(DefaultBroadPhase::new()),
					narrow_phase: Box::new(DefaultNarrowPhase::new()),
					collision_resolution: Box::new(DefaultCollisionResolution::new()),
					bodies: Vec::new() }
	}

	pub fn add_body(&mut self, body: Body) {
		self.bodies.push(body);
	}

	pub fn clear(&mut self) {
		self.bodies.clear();
	}

	pub fn step(&mut self, timeStep: f32) {
		let len = self.bodies.len();
		for i in 0..len {
			self.bodies[i].integrate(timeStep);
		}

		let pairs = self.broad_phase.run(&self.bodies);
		let collider_results = self.narrow_phase.run(&pairs);
		let mut manifolds: Vec<Manifold> = Vec::new();
		for result in collider_results.iter() {
			if result.is_colliding {
				match result.manifold {
					Some(ref manifold) => {
						manifolds.push((*manifold).clone());
					},
					None => {
						//Do nothing
					}
				}
			}
		}
		self.collision_resolution.run(&manifolds);
	}
}
