use super::collision_resolution::CollisionResolution;
use super::manifold::Manifold;
use super::math::Vec2;

pub struct DefaultCollisionResolution;

impl CollisionResolution for DefaultCollisionResolution {
    fn run(&self, manifolds: &Vec<Manifold>) {
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
                    body_a.velocity = body_a.velocity - impulse.multiply(1.0 / body_a.mass);
                } if body_b.mass != 0.0 {
                    body_b.velocity = body_b.velocity - impulse.multiply(1.0 / body_b.mass);
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
                    body_a.position = body_a.position + correction.multiply(body_a_inv_mass);
                }

                if !body_b_inv_mass.is_infinite() && !body_b_inv_mass.is_nan() && body_b_inv_mass > 0.0 {
                    body_b.position = body_b.position - correction.multiply(body_b_inv_mass);
                }
            }
        }
    }
}

impl DefaultCollisionResolution {
    pub fn new() -> DefaultCollisionResolution {
        return DefaultCollisionResolution
    }
}
