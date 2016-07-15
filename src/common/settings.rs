use std::f32::consts::PI;

// Collision

/// The maximum number of contact points between two convex shapes. Do
/// not change this value.
const MAX_MANIFOLD_POINTS: i32 = 2;

/// The maximum number of vertices on a convex polygon.
const MAX_POLYGON_VERTICES: i32 = 8;

/// The is used to flatten AABBs in the dynamic tree. This allows proxies
/// to move by a small amount without triggering a tree adjustment.
/// This is in meters.
const AABB_EXTENSION: f32 = 0.1;

/// This is used to fatten AABBs in the dynamic tree. This is used to predict
/// the future position based on the current displacement.
/// This is a dimenionless multiplier.
const AABB_MULTIPLIER: f32 = 2.0;

/// A small length used as a collision and constraint tolerance. Usually it is
/// chosen to be numerically significant, but visually insignificant.
const LINEAR_SLOP: f32 = 0.005;

/// A small angle used as a collision and constraint tolerance. Usually it is
/// chosen to be numerically significant, but visually insignificant.
const ANGULAR_SLOP: f32 = (2.0 / 180.0 * PI);

/// The radius of the polygon/edge shape skin. This should not be modified. Making
/// this smaller means polygons will have an insufficient buffer for continuous collision.
/// Making it larger may create artifacts for vertex collision.
const POLYGON_RADIUS: f32 = (2.0 * LINEAR_SLOP);

/// Maximum number of sub-steps per contact in continuous physics simulation.
const MAX_SUB_STEPS: i32 = 8;

/// Dynamics

/// Maximum number of contacts to be handled to solve a TOI impact.
const MAX_TOI_CONTACTS: i32 = 32;
