use super::super::common::Vec2;
use super::body_type::BodyType;

/// A body definition holds all the data needed to construct a rigid body.
/// You can safely re-use body definitions. Shapes are added to a body after construction.
pub struct BodyDef {
    /// The body type: static, kinematic, or dynamic.
	/// Note: if a dynamic body would have zero mass, the mass is set to one.
    pub typ: BodyType,
    /// The world position of the body. Avoid creating bodies at the origin
	/// since this can lead to many overlapping shapes.
    pub position: Vec2,
    /// The world angle of the body in radians.
    pub angle: f32,
    /// The linear velocity of the body's origin in world co-ordinates.
    pub linear_velocity: Vec2,
    /// The angular velocity of the body.
    pub angular_velocity: f32,
    /// Linear damping is use to reduce the linear velocity. The damping parameter
	/// can be larger than 1.0f but the damping effect becomes sensitive to the
	/// time step when the damping parameter is large.
    pub linear_dampling: f32,
    /// Angular damping is use to reduce the angular velocity. The damping parameter
	/// can be larger than 1.0f but the damping effect becomes sensitive to the
	/// time step when the damping parameter is large.
    pub angular_damping: f32,
    /// Set this flag to false if this body should never fall asleep. Note that
	/// this increases CPU usage.
    pub allow_sleep: bool,
    /// Is this body initially awake or sleeping?
    pub awake: bool,
    /// Should this body be prevented from rotating? Useful for characters.
    pub fixed_rotation: bool,
    /// Is this a fast moving body that should be prevented from tunneling through
	/// other moving bodies? Note that all bodies are prevented from tunneling through
	/// kinematic and static bodies. This setting is only considered on dynamic bodies.
	/// @warning You should use this flag sparingly since it increases processing time.
    pub bullet: bool,
    /// Does this body start out active?
    pub active: bool,
    // TODO user_data
    pub gravity_scale: f32
}

impl BodyDef {
    pub fn new() -> BodyDef {
        BodyDef {
            typ: BodyType::StaticBody,
            position: Vec2::zero(),
            angle: 0.0,
            linear_velocity: Vec2::zero(),
            angular_velocity: 0.0,
            linear_dampling: 0.0,
            angular_damping: 0.0,
            allow_sleep: true,
            awake: true,
            fixed_rotation: false,
            bullet: false,
            active: true,
            // TODO user_data
            gravity_scale: 1.0
        }
    }
}
