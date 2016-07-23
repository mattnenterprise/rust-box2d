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

    // TODO implement {
    // pub fn get_search_direction() -> Vec2

    // TODO implement
    // pub fn get_closest_point() -> Vec2

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

    // TODO implement
    pub fn solve_2() {

    }

    // TODO implement
    pub fn solve_3() {

    }
}
