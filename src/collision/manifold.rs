use super::manifold_point::ManifoldPoint;
use super::super::common::Vec2;
use super::super::common::settings::MAX_MANIFOLD_POINTS;

pub enum ManifoldType {
    Circles,
    FaceA,
    FaceB
}

/// A manifold for two touching convex shapes.
/// Box2D supports multiple types of contact:
/// - clip point versus plane with radius
/// - point versus point with radius (circles)
/// The local point usage depends on the manifold type:
/// -e_circles: the local center of circleA
/// -e_faceA: the center of faceA
/// -e_faceB: the center of faceB
/// Similarly the local normal usage:
/// -e_circles: not used
/// -e_faceA: the normal on polygonA
/// -e_faceB: the normal on polygonB
/// We store contacts in this way so that position correction can
/// account for movement, which is critical for continuous physics.
/// All contact scenarios must be expressed in one of these types.
/// This structure is stored across time steps, so we keep it small.
pub struct Manifold {
    /// The points of contact
    pub points: [ManifoldPoint; MAX_MANIFOLD_POINTS as usize],
    /// Not use for Type::e_points
    pub local_normal: Vec2,
    /// Usage depends on manifold type
    pub local_point: Vec2,
    pub typ: ManifoldType,
    /// The number of manifold points
    pub point_count: i32
}

impl Manifold {
    pub fn new() -> Manifold {
        Manifold {
            points: [ManifoldPoint::new(); MAX_MANIFOLD_POINTS as usize],
            local_normal: Vec2::zero(),
            local_point: Vec2::zero(),
            typ: ManifoldType::Circles,
            point_count: 0,
        }
    }
}
