use super::math::Vec2;
use super::body::BodyDef;
use super::body::Body;
use super::manifold::Manifold;
use super::broad_phase::BroadPhase;
use super::default_broad_phase::DefaultBroadPhase;
use super::narrow_phase::NarrowPhase;
use super::default_narrow_phase::DefaultNarrowPhase;
use super::collision_resolution::CollisionResolution;

pub struct World {
	gravity: Vec2,
	broad_phase: Box<BroadPhase>,
	narrow_phase: Box<NarrowPhase>,
	pub bodies: Vec<Body>
}

impl World {
	pub fn new(gravity: Vec2) -> World {
		return World{ gravity: gravity,
					broad_phase: Box::new(DefaultBroadPhase::new()),
					narrow_phase: Box::new(DefaultNarrowPhase::new()),
					bodies: Vec::new() }
	}

	pub fn add_body(&mut self, body_def: BodyDef) {
		let id = self.bodies.len();
		let body = Body::new(id, body_def);
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
		self.resolve_collisions(&manifolds);
	}
}

impl CollisionResolution for World {
    fn resolve_collisions(&mut self, manifolds: &Vec<Manifold>) {
        for m in manifolds.iter() {
            let manifold = m.clone();
            let mut body_a = manifold.body_a;
            let mut body_b = manifold.body_b;
            let mut rv = body_b.velocity - body_a.velocity;
            let vel_along_normal = rv.dot(manifold.normal);

            if vel_along_normal > 0.0 {
                let a_restitution = body_a.restitution;
                let b_restitution = body_b.restitution;
                let e = a_restitution.min(b_restitution);

                let mut j = vel_along_normal * (-(1.0 + e));
                if body_a.mass != 0.0 && body_b.mass != 0.0 {
                    j /= 1.0 / body_a.mass + (1.0 / body_b.mass);
                } else if body_a.mass != 0.0 && body_b.mass == 0.0 {
                    j /= 1.0 / body_a.mass;
                } else if body_a.mass == 0.0 && body_b.mass != 0.0 {
                    j /= 1.0 / body_b.mass;
                }

                let impulse = manifold.normal.multiply(j);
                if body_a.mass != 0.0 {
					self.bodies[body_a.id].velocity = body_a.velocity - impulse.multiply(1.0 / body_a.mass);
                } if body_b.mass != 0.0 {
					self.bodies[body_b.id].velocity = body_b.velocity - impulse.multiply(1.0 / body_b.mass);
                }

                let k_slop = 0.01;
                let percent = 0.5;
                let maximum = (manifold.penetration - k_slop).max(0.0);
                let body_a_inv_mass = 1.0 / body_a.mass;
                let body_b_inv_mass = 1.0 / body_b.mass;
                let mut correction = Vec2::new(0.0, 0.0);
                if body_b_inv_mass.is_infinite() || body_b_inv_mass.is_nan() {
                    correction = manifold.normal.multiply(maximum / (body_a_inv_mass) * percent);
                } else if body_a_inv_mass.is_infinite() || body_a_inv_mass.is_nan() {
                    correction = manifold.normal.multiply(maximum / (body_b_inv_mass) * percent);
                } else {
                    correction = manifold.normal.multiply(maximum / (body_a_inv_mass + body_b_inv_mass) * percent);
                }

                if body_a_inv_mass > 0.0 {
					self.bodies[body_a.id].position = body_a.position + correction.multiply(body_a_inv_mass);
                }

                if !body_b_inv_mass.is_infinite() && !body_b_inv_mass.is_nan() && body_b_inv_mass > 0.0 {
					self.bodies[body_b.id].position = body_b.position - correction.multiply(body_b_inv_mass);
                }
            }
        }
    }
}
