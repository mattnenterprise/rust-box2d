use super::simplex_vertex::SimplexVertex;
use super::super::common::math::vec_distance;
use super::super::common::Vec2;

pub struct Simplex {
    pub m_v1: SimplexVertex,
    pub m_v2: SimplexVertex,
    pub m_v3: SimplexVertex,
    pub m_count: i32
}

impl Simplex {
    // TODO implement
    // pub fn read_cache()

    // TODO implement
    // pub fn write_cache()

    pub fn get_search_direction(&self) -> Vec2 {
        match self.m_count {
            1 => {
                return -self.m_v1.w;
            },
            2 => {
                let e12 = self.m_v2.w - self.m_v1.w;
                let sgn = Vec2::cross_vec(e12, -self.m_v1.w);
                if sgn > 0.0 {
                    // Origin is left of e12.
                    return Vec2::cross_scalar_vec(1.0, e12);
                } else {
                    // Origin is right of e12.
					return Vec2::cross_vec_scalar(e12, 1.0);
                }
            },
            _ => {
                assert!(false);
                return Vec2::zero();
            }
        }
    }

    pub fn get_closest_point(&self) -> Vec2 {
        match self.m_count {
            0 => {
                assert!(false);
                return Vec2::zero();
            },
            1 => {
                return self.m_v1.w;
            },
            2 => {
                return self.m_v1.a * self.m_v1.w + self.m_v2.a * self.m_v2.w;
            },
            3 => {
                return Vec2::zero();
            },
            _ => {
                assert!(false);
                return Vec2::zero();
            }
        }
    }

    pub fn get_witness_points(&self) -> (Vec2, Vec2) {
        match self.m_count {
            0 => {
                assert!(false);
                return (Vec2::zero(), Vec2::zero());
            },
            1 => {
                return (self.m_v1.w_a, self.m_v1.w_b);
            },
            2 => {
                return (self.m_v1.a * self.m_v1.w_a + self.m_v2.a * self.m_v2.w_a,
                    self.m_v1.a * self.m_v1.w_b + self.m_v2.a * self.m_v2.w_b)
            },
            3 => {
                let p_a = self.m_v1.a * self.m_v1.w_a + self.m_v2.a * self.m_v2.w_a + self.m_v3.a * self.m_v3.w_a;
                return (p_a, p_a)
            },
            _ => {
                assert!(false);
                return (Vec2::zero(), Vec2::zero());
            }
        }
    }

    pub fn get_metric(&self) -> f32 {
        match self.m_count {
            0 => {
                assert!(false);
                return 0.0;
            },
            1 => 0.0,
            2 => {
                return vec_distance(self.m_v1.w, self.m_v2.w);
            },
            3 => {
                return Vec2::cross_vec(self.m_v2.w - self.m_v1.w, self.m_v3.w - self.m_v1.w);
            },
            _ => {
                assert!(false);
                return 0.0;
            }
        }
    }

    // Solve a line segment using barycentric coordinates.
    //
    // p = a1 * w1 + a2 * w2
    // a1 + a2 = 1
    //
    // The vector from the origin to the closest point on the line is
    // perpendicular to the line.
    // e12 = w2 - w1
    // dot(p, e) = 0
    // a1 * dot(w1, e) + a2 * dot(w2, e) = 0
    //
    // 2-by-2 linear system
    // [1      1     ][a1] = [1]
    // [w1.e12 w2.e12][a2] = [0]
    //
    // Define
    // d12_1 =  dot(w2, e12)
    // d12_2 = -dot(w1, e12)
    // d12 = d12_1 + d12_2
    //
    // Solution
    // a1 = d12_1 / d12
    // a2 = d12_2 / d12
    pub fn solve_2(&mut self) {
        let w1 = self.m_v1.w;
        let w2 = self.m_v2.w;
        let e12 = w2 - w1;

        // w1 region
        let d12_2 = - Vec2::dot(w1, e12);
        if d12_2 <= 0.0 {
            // a2 <= 0, so we clamp it to 0
            self.m_v1.a = 1.0;
            self.m_count = 1;
            return;
        }

        // w2 region
        let d12_1 = Vec2::dot(w2, e12);
        if d12_1 <= 0.0 {
            // a1 <= 0, so we clamp it to 0
            self.m_v1.a = 1.0;
            self.m_count = 1;
            self.m_v1 = self.m_v2.clone();
            return;
        }

        let inv_d12 = 1.0 / (d12_1 + d12_2);
        self.m_v1.a = d12_1 * inv_d12;
        self.m_v2.a = d12_2 * inv_d12;
        self.m_count = 2;
    }

    // Possible regions:
    // - points[2]
    // - edge points[0]-points[2]
    // - edge points[1]-points[2]
    // - inside the triangle
    pub fn solve_3(&mut self) {
        let w1 = self.m_v1.w;
        let w2 = self.m_v2.w;
        let w3 = self.m_v3.w;

        // Edge12
    	// [1      1     ][a1] = [1]
    	// [w1.e12 w2.e12][a2] = [0]
    	// a3 = 0
    	let e12 = w2 - w1;
    	let w1e12 = Vec2::dot(w1, e12);
    	let w2e12 = Vec2::dot(w2, e12);
    	let d12_1 = w2e12;
    	let d12_2 = -w1e12;

        // Edge13
    	// [1      1     ][a1] = [1]
    	// [w1.e13 w3.e13][a3] = [0]
    	// a2 = 0
    	let e13 = w3 - w1;
    	let w1e13 = Vec2::dot(w1, e13);
    	let w3e13 = Vec2::dot(w3, e13);
    	let d13_1 = w3e13;
    	let d13_2 = -w1e13;

        // Edge23
    	// [1      1     ][a2] = [1]
    	// [w2.e23 w3.e23][a3] = [0]
    	// a1 = 0
    	let e23 = w3 - w2;
    	let w2e23 = Vec2::dot(w2, e23);
    	let w3e23 = Vec2::dot(w3, e23);
    	let d23_1 = w3e23;
    	let d23_2 = -w2e23;

        // Triangle123
    	let n123 = Vec2::cross_vec(e12, e13);

    	let d123_1 = n123 * Vec2::cross_vec(w2, w3);
    	let d123_2 = n123 * Vec2::cross_vec(w3, w1);
    	let d123_3 = n123 * Vec2::cross_vec(w1, w2);

        // w1 region
    	if d12_2 <= 0.0 && d13_2 <= 0.0 {
    		self.m_v1.a = 1.0;
    		self.m_count = 1;
    		return;
    	}

        // e12
    	if d12_1 > 0.0 && d12_2 > 0.0 && d123_3 <= 0.0 {
    		let inv_d12 = 1.0 / (d12_1 + d12_2);
    		self.m_v1.a = d12_1 * inv_d12;
    		self.m_v2.a = d12_2 * inv_d12;
    		self.m_count = 2;
    		return;
    	}

        // e13
    	if d13_1 > 0.0 && d13_2 > 0.0 && d123_2 <= 0.0 {
    		let inv_d13 = 1.0 / (d13_1 + d13_2);
    		self.m_v1.a = d13_1 * inv_d13;
    		self.m_v3.a = d13_2 * inv_d13;
    		self.m_count = 2;
    		self.m_v2 = self.m_v3.clone();
    		return;
    	}

        // w2 region
    	if d12_1 <= 0.0 && d23_2 <= 0.0 {
    		self.m_v2.a = 1.0;
    		self.m_count = 1;
    		self.m_v1 = self.m_v2.clone();
    		return;
    	}

        // w3 region
    	if d13_1 <= 0.0 && d23_1 <= 0.0 {
    		self.m_v3.a = 1.0;
    		self.m_count = 1;
    		self.m_v1 = self.m_v3.clone();
    		return;
    	}

        // e23
    	if d23_1 > 0.0 && d23_2 > 0.0 && d123_1 <= 0.0 {
    		let inv_d23 = 1.0 / (d23_1 + d23_2);
    		self.m_v2.a = d23_1 * inv_d23;
    		self.m_v3.a = d23_2 * inv_d23;
    		self.m_count = 2;
    		self.m_v1 = self.m_v3.clone();
    		return;
    	}

        // Must be in triangle123
    	let inv_d123 = 1.0 / (d123_1 + d123_2 + d123_3);
    	self.m_v1.a = d123_1 * inv_d123;
    	self.m_v2.a = d123_2 * inv_d123;
    	self.m_v3.a = d123_3 * inv_d123;
    	self.m_count = 3;
    }
}
