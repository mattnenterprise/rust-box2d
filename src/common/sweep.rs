use super::Rot;
use super::Transform;
use super::Vec2;
use super::math::mul_rot_vec2;
use std::f32::consts::PI;

/// This describes the motion of a body/shape for TOI computation.
/// Shapes are defined with respect to the body origin, which may
/// no coincide with the center of mass. However, to support dynamics
/// we must interpolate the center of mass position.
pub struct Sweep {
    /// local center of mass position
    pub local_center: Vec2,
    /// center world positions
    pub c0: Vec2,
    pub c: Vec2,
    /// world angles
    pub a0: f32,
    pub a: f32,
    /// Fraction of the current time step in the range [0,1]
	/// c0 and a0 are the positions at alpha0.
    pub alpha0: f32
}

impl Sweep {
    pub fn new() -> Sweep {
        Sweep {
            local_center: Vec2::zero(),
            c0: Vec2::zero(),
            c: Vec2::zero(),
            a0: 0.0,
            a: 0.0,
            alpha0: 0.0
        }
    }

    /// Get the interpolated transform at a specific time.
	/// @param beta is a factor in [0,1], where 0 indicates alpha0.
    pub fn get_transform(&mut self, beta: f32) -> Transform {
        let angle = (1.0 - beta) * self.a0 + beta * self.a;
        let rot = Rot::new_angle(angle);
        let p = ((1.0 - beta) * self.c0 + beta * self.c) - mul_rot_vec2(Rot::new_angle(angle), self.local_center);
        Transform::new(p, rot)
    }

    /// Advance the sweep forward, yielding a new initial state.
	/// @param alpha the new initial time.
    pub fn advance(&mut self, alpha: f32) {
        assert!(self.alpha0 < 1.0);
        let beta = (alpha - self.alpha0) / (1.0 / self.alpha0);
        self.c0 += (self.c - self.c0) * beta;
        self.a0 += beta * (self.a - self.a0);
        self.alpha0 = alpha;
    }

    /// Normalize the angles.
    pub fn normalize(&mut self) {
        let two_pi = 2.0 * PI;
        let d = two_pi * (self.a0 / two_pi).floor();
        self.a0 -= d;
        self.a -= d;
    }
}
