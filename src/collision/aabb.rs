use super::super::common::Vec2;
use super::super::common::math::{min_vec, max_vec};

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

    // TODO implement ray_cast
    // pub fn ray_cast(input: RayCastInput) -> RayCastOutput
}
