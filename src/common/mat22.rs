use super::Vec2;

/*
	Has the following format
	[a11 a12]
	[a21 a22]
	Stored in column-major order.
*/
pub struct Mat22 {
	pub ex: Vec2,
	pub ey: Vec2
}

impl Mat22 {
	pub fn new() -> Mat22 {
		Mat22::new_scalars(0.0, 0.0, 0.0, 0.0)
	}

    pub fn new_columns(c1: Vec2, c2: Vec2) -> Mat22 {
        Mat22 {
            ex: c1,
            ey: c2
        }
    }

    pub fn new_scalars(a11: f32, a12: f32, a21: f32, a22: f32) -> Mat22 {
        Mat22 {
            ex: Vec2::new(a11, a21),
            ey: Vec2::new(a12, a22)
        }
    }

    pub fn set(&mut self, c1: Vec2, c2: Vec2) {
        self.ex = c1;
        self.ey = c2;
    }

    pub fn set_identity(&mut self) {
        self.ex.x = 1.0; self.ey.x = 0.0;
        self.ex.y = 0.0; self.ey.y = 1.0;
    }

    pub fn set_zero(&mut self) {
        self.ex.x = 0.0; self.ey.x = 0.0;
        self.ex.y = 0.0; self.ey.y = 0.0;
    }

    pub fn get_inverse(self) -> Mat22 {
        let a = self.ex.x;
        let b = self.ey.x;
        let c = self.ex.y;
        let d = self.ey.y;
        let mut det = a * d - b * c;
		if det != 0.0 {
			det = 1.0 / det;
		}
        Mat22::new_scalars(det * d, -det * c, -det * b, det * a)
    }

    /// Solve A * x = b, where b is a column vector. This is more efficient
    /// than computing the inverse in one-shot-cases.
    pub fn solve(self, b: Vec2) -> Vec2 {
        let a11 = self.ex.x;
        let a12 = self.ey.x;
        let a21 = self.ex.y;
        let a22 = self.ey.y;
		let mut det = a11 * a22 - a12 * a21;
		if det != 0.0 {
			det = 1.0 / det;
		}
        let x = det * (a22 * b.x - a12 * b.y);
        let y = det * (a11 * b.y - a21 * b.x);
        Vec2::new(x, y)
    }
}
