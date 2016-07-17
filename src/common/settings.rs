use std::f32::consts::PI;

/// Collision

/// The maximum number of contact points between two convex shapes. Do
/// not change this value.
pub const MAX_MANIFOLD_POINTS: i32 = 2;

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

/// A velocity threshold for elastic collisions. Any collisions with a relative linear
/// velocity below this threshold will be treated as inelastic.
const VELOCITY_THRESHOLD: f32 = 1.0;

/// The maximum linear position correction used when solving constraints. This helps to
/// prevent overshoot.
const MAX_LINEAR_CORRECTION: f32 = 0.2;

/// The maximum linear velocity of a body. This limit is very large and is used
/// to prevent numerical problems. You shouldn't need to adjust this.
const MAX_TRANSLATION: f32 = 2.0;
const MAX_TRANSLATION_SQUARED: f32 = MAX_TRANSLATION * MAX_TRANSLATION;

/// The maximum linear velocity of a body. This limit is very large and is used
/// to prevent numerical problems. You shouldn't need to adjust this.
const MAX_ROTATION: f32 = 0.5 * PI;
const MAX_ROTATION_SQUARED: f32 = MAX_ROTATION * MAX_ROTATION;

/// This scale factor controls how fast overlap is resolved. Ideally this would be 1 so
/// that overlap is removed in one time step. However using values close to 1 often lead
/// to overshoot.
const BAUMGARTE: f32 = 0.2;
const TOI_BAUGARTE: f32 = 0.75;

/// Sleep

/// The time that a body must be still before it will go to sleep.
const TIME_TO_SLEEP: f32 = 0.5;

/// A body cannot sleep if its linear velocity is above this tolerance.
const LINEAR_SLEEP_TOLERANCE: f32 = 0.01;

/// A body cannot sleep if its angular velocity is above this tolerance.
const ANGULAR_SLEEP_TOLERANCE: f32 = (2.0 / 180.0 * PI);
