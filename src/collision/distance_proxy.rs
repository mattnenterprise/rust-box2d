use super::super::common::Vec2;
use super::shapes::shape::Shape;
use super::shapes::shape::ShapeType;
use super::shapes::to_derived_shape::ToDerivedShape;

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

    pub fn set<T: Shape+ToDerivedShape>(&mut self, shape: T, index: i32)  {
        match shape.get_type() {
            ShapeType::Circle => {
                let circle = shape.circle().unwrap();
                self.m_vertices.push(circle.m_p);
                self.m_radius = circle.m_radius;
            },
            ShapeType::Polygon => {
                let polygon = shape.polygon().unwrap();
                for vertex in polygon.m_vertices.iter() {
                    self.m_vertices.push(*vertex);
                }
                self.m_radius = polygon.m_radius;
            },
            ShapeType::Chain => {
                let chain = shape.chain().unwrap();
                assert!(0 <= index && index < (chain.m_vertices.len() as i32));
                self.m_buffer[0] = chain.m_vertices[index as usize];
                if index + 1 < (chain.m_vertices.len() as i32) {
                    self.m_buffer[1] = chain.m_vertices[(index + 1) as usize];
                } else {
                    self.m_buffer[1] = chain.m_vertices[0 as usize];
                }
                for buf in self.m_buffer.iter() {
                    self.m_vertices.push(*buf);
                }
                self.m_radius = chain.m_radius;
            },
            ShapeType::Edge => {
                let edge = shape.edge().unwrap();
                self.m_vertices.push(edge.m_vertex1);
                self.m_vertices.push(edge.m_vertex2);
                self.m_radius = edge.m_radius;
            }
        }
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
