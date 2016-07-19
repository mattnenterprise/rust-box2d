use super::super::common::Vec2;
use super::shapes::shape::Shape;

pub struct DistanceProxy {
    pub m_buffer: [Vec2; 2],
    pub m_vertices: Vec<Vec2>,
    pub m_radius: f32
}

impl DistanceProxy {
    pub fn new() -> DistanceProxy {
        DistanceProxy {
            m_buffer: [Vec2::zero(); 2],
            m_vertices: Vec::new(),
            m_radius: 0.0
        }
    }

    // TODO implement
    pub fn set<T: Shape>(_: T, _: i32)  {

    }

    pub fn get_vertex_count(self) -> i32 {
        self.m_vertices.len() as i32
    }

    pub fn get_vertex(&self, index: i32) -> Vec2 {
        assert!(0 <= index && (index as usize) < self.m_vertices.len());
        self.m_vertices[index as usize]
    }

    pub fn get_support(&self, d: Vec2) -> i32 {
        let mut best_index = 0;
        let mut best_value = Vec2::dot(self.m_vertices[0], d);
        for (i, v) in self.m_vertices.iter().enumerate() {
            let value = Vec2::dot(*v, d);
            if value > best_value {
                best_index = i;
                best_value = value;
            }
        }
        return best_index as i32;
    }

    pub fn get_support_vertex(&self, d: Vec2) -> Vec2 {
        let mut best_index = 0;
        let mut best_value = Vec2::dot(self.m_vertices[0], d);
        for (i, v) in self.m_vertices.iter().enumerate() {
            let value = Vec2::dot(*v, d);
            if value > best_value {
                best_index = i;
                best_value = value;
            }
        }
        return self.m_vertices[best_index];
    }
}
