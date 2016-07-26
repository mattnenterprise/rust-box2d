use super::body_type::BodyType;
use super::super::common::Transform;
use super::super::common::Sweep;
use super::super::common::Vec2;

pub struct Body {
    pub m_type: BodyType,
    pub m_flags: u16,
    pub m_island_index: i32,
    pub m_xf: Transform,
    pub m_sweep: Sweep,
    pub m_linear_velocity: Vec2,
    pub m_angular_velocity: f32,
    pub m_force: Vec2,
    pub m_torque: f32
}
