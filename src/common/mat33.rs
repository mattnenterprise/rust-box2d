use super::Vec2;
use super::Vec3;

/// A 3-by-3 matrix. Stored in column-major order.
pub struct Mat33 {
    pub ex: Vec3,
    pub ey: Vec3,
    pub ez: Vec3
}

impl Mat33 {
    pub fn new() -> Mat33 {
        Mat33 {
            ex: Vec3::new(0.0, 0.0, 0.0),
            ey: Vec3::new(0.0, 0.0, 0.0),
            ez: Vec3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn new_columns(c1: Vec3, c2: Vec3, c3: Vec3) -> Mat33 {
        Mat33 {
            ex: c1,
            ey: c2,
            ez: c3
        }
    }

    /// Set this matrix to all zeros.
    pub fn set_zero(&mut self) {
        self.ex.set_zero();
        self.ey.set_zero();
        self.ez.set_zero();
    }

    /// Solve A * x = b, where b is a column vector. This is more efficient
    /// than computing the inverse in one-shot cases.
    pub fn solve33(&self, b: Vec3) -> Vec3 {
        let mut det = Vec3::dot(self.ex, Vec3::cross(self.ey, self.ez));
        if det != 0.0 {
            det = 1.0 / det;
        }
        let x = det * Vec3::dot(b, Vec3::cross(self.ey, self.ez));
        let y = det * Vec3::dot(self.ex, Vec3::cross(b, self.ez));
        let z = det * Vec3::dot(self.ex, Vec3::cross(self.ey, b));
        Vec3::new(x, y, z)
    }

    /// Solve A * x = b, where b is a column vector. This is more efficient
    /// than computing the inverse in one-shot cases.
    pub fn solve22(&self, b: Vec2) -> Vec2 {
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

    ///
    pub fn get_inverse22(&self) -> Mat33 {
        let a = self.ex.x;
        let b = self.ey.x;
        let c = self.ex.y;
        let d = self.eu.y;
        let mut det = a * d - b * c;
        if det != 0.0 {
            det = 1.0 / det;
        }
        let m = Mat33::new();
        m.ex.x =  det * d;	m.ey.x = -det * b; m.ex.z = 0.0;
        m.ex.y = -det * c;	m.ey.y =  det * a; m.ey.z = 0.0;
        m.ez.x = 0.0; m.ez.y = 0.0; m.ez.z = 0.0;
        return m
    }
}
