use crate::math::{fast_cos, fast_sin};
use crate::vectors::vector2::Vector2;

/// A 2D angle for representing a rotation in 2d space.
pub struct Angle2 {
    radians: f32,
}

impl Angle2 {

    /// Creates a new angle from degrees.
    pub fn from_degrees(degrees: f32) -> Self {
        Self { radians: degrees.to_radians() }
    }

    /// Returns the angle in degrees.
    pub fn to_degrees(&self) -> f32 {
        self.radians.to_degrees()
    }

    /// Creates a new angle from radians.
    pub fn from_radians(radians: f32) -> Self {
        Self { radians }
    }

    /// Returns the angle in radians.
    pub fn to_radians(&self) -> f32 {
        self.radians
    }

    /// Takes `vector` and rotates it by this angle.
    /// Returns the rotated Vector.
    pub fn rotate_vector(&self, vector: Vector2) -> Vector2 {
        let cos_theta = fast_cos(self.radians);
        let sin_theta = fast_sin(self.radians);
        Vector2::new(cos_theta * vector.x - sin_theta * vector.y, sin_theta * vector.x + cos_theta * vector.y)
    }

}