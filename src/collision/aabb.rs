use super::super::common::Vec2;
use super::super::common::math::{min_vec, max_vec, abs_vec};
use super::ray_cast_input::RayCastInput;
use super::ray_cast_output::RayCastOutput;
use std::f32;

pub struct AABB {
    /// The lower vertex
    pub lower_bound: Vec2,
    /// The upper vertex
    pub upper_bound: Vec2
}

impl AABB {
    pub fn new() -> AABB {
        AABB {
            lower_bound: Vec2::zero(),
            upper_bound: Vec2::zero()
        }
    }

    /// Verify that the bounds are sorted.
    pub fn is_valid(self) -> bool {
        let d = self.upper_bound - self.lower_bound;
        let mut valid = d.x >= 0.0 && d.y >= 0.0;
        valid = valid && self.lower_bound.is_valid() && self.upper_bound.is_valid();
        return valid;
    }

    /// Get the center of the AABB.
    pub fn get_center(self) -> Vec2 {
        0.5 * (self.lower_bound + self.upper_bound)
    }

    /// Get the extents of the AABB (half-widths).
    pub fn get_extents(self) -> Vec2 {
        0.5 * (self.upper_bound - self.lower_bound)
    }

    /// Get the perimeter length
    pub fn get_perimeter(self) -> f32 {
        let wx = self.upper_bound.x - self.lower_bound.x;
        let wy = self.upper_bound.y - self.upper_bound.y;
        2.0 * (wx + wy)
    }

    /// Combine an AABB into this one.
    pub fn combine(&mut self, aabb: AABB) {
        self.lower_bound = min_vec(self.lower_bound, aabb.lower_bound);
        self.upper_bound = max_vec(self.upper_bound, aabb.upper_bound);
    }

    /// Combine two AABBs into this one
    pub fn combine_two(&mut self, aabb1: AABB, aabb2: AABB) {
        self.lower_bound = min_vec(aabb1.lower_bound, aabb2.lower_bound);
        self.upper_bound = max_vec(aabb1.upper_bound, aabb2.upper_bound);
    }

    /// Does this aabb contain the procided AABB.
    pub fn contains(&mut self, aabb: AABB) -> bool {
        let mut result = true;
        result = result && self.lower_bound.x <= aabb.lower_bound.x;
        result = result && self.lower_bound.y <= aabb.lower_bound.y;
        result = result && aabb.upper_bound.x <= self.upper_bound.x;
        result = result && aabb.upper_bound.y <= self.upper_bound.y;
        return result;
    }

    // TODO Does this work ?
    // From Real-time Collision Detection, p179.
    pub fn ray_cast(&self, input: RayCastInput) -> Option<RayCastOutput> {
        let mut tmin = -f32::MAX;
        let mut tmax = f32::MAX;

        let p = input.p1;
        let d = input.p2 - input.p1;
        let abs_d = abs_vec(d);

        let mut normal = Vec2::zero();

        // Do x
        if abs_d.x < f32::EPSILON {
			// Parallel.
			if p.x < self.lower_bound.x || self.upper_bound.x < p.x {
				return None;
			}
		} else {
			let inv_d = 1.0 / d.x;
			let mut t1 = (self.lower_bound.x - p.x) * inv_d;
			let mut t2 = (self.upper_bound.x - p.x) * inv_d;

			// Sign of the normal vector.
			let mut s = -1.0;

			if t1 > t2
			{
                let tmp = t1;
                t1 = t2;
                t2 = tmp;
				s = 1.0;
			}

			// Push the min up
			if t1 > tmin {
				normal.set_zero();
				normal.x = s;
				tmin = t1;
			}

			// Pull the max down
			tmax = tmax.min(t2);

			if tmin > tmax {
				return None;
			}
		}



        // Do y
        if abs_d.y < f32::EPSILON {
			// Parallel.
			if p.y < self.lower_bound.y || self.upper_bound.y < p.y {
				return None;
			}
		} else {
			let inv_d = 1.0 / d.y;
			let mut t1 = (self.lower_bound.y - p.y) * inv_d;
			let mut t2 = (self.upper_bound.y - p.y) * inv_d;

			// Sign of the normal vector.
			let mut s = -1.0;

			if t1 > t2
			{
                let tmp = t1;
                t1 = t2;
                t2 = tmp;
				s = 1.0;
			}

			// Push the min up
			if t1 > tmin {
				normal.set_zero();
				normal.y = s;
				tmin = t1;
			}

			// Pull the max down
			tmax = tmax.min(t2);

			if tmin > tmax {
				return None;
			}
		}

        if tmin < 0.0 || input.max_fraction < tmin {
            return None;
        }

        Some(RayCastOutput {
            fraction: tmin,
            normal: normal
        })
    }
}

/// Determine if AABB overlap
pub fn test_aabb_overlap(a: AABB, b: AABB) -> bool {
    let d1 = b.lower_bound - a.upper_bound;
    let d2 = a.lower_bound - b.upper_bound;

    if d1.x > 0.0 || d1.y > 0.0 {
        return false;
    }

    if d2.x > 0.0 || d2.y > 0.0 {
        return false;
    }

    return true;
}
