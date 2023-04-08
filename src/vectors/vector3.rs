use std::ops::{Add, Div, Mul, Sub};
use crate::math::fast_inv_sqrt;
use crate::angles::quaternion::Quaternion;

/// A vector with x, y, and z components.
/// They are used to represent a point or direction in 3d space.
#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {

    /// Creates a new vector with the given x, y, and z components.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Creates a new vector with all components set to 0.
    #[inline]
    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    /// Creates a new vector with all components set to 1.
    #[inline]
    pub fn one() -> Vector3 {
        Vector3::new(1.0, 1.0, 1.0)
    }

    /// Creates a new vector with all components set to the given value.
    #[inline]
    pub fn from_one(x: f32) -> Vector3 {
        Vector3::new(x, x, x)
    }

    /// Returns the dot product of this and other vector.
    #[inline]
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> f32 {
        fast_inv_sqrt(self.magnitude_squared())
    }

    /// Returns the squared magnitude of this vector.
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns a normalized copy of this vector.
    #[inline]
    pub fn normalized(self) -> Vector3 {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            self
        } else {
            Vector3 {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
            }
        }
    }

    /// Reflects the vector about the given normal.
    #[inline]
    pub fn reflect(self, normal: Vector3) -> Self {
        normal.scale(&self.dot(&normal) * 2.0) - self
    }

    /// Projects the vector onto the vector other.
    #[inline]
    pub fn project(&self, other: Vector3) -> Self {
        other.scale(self.dot(&other) /  other.magnitude_squared())
    }

    /// Scales this vector by the given scalar.
    #[inline]
    pub fn scale(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Converts this Vector into a Quaternion using the given scalar (w) component.
    #[inline]
    pub fn to_quaternion(&self, w: f32) -> Quaternion {
        Quaternion::new(w, self.x, self.y, self.z)
    }

    pub fn as_bytes(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        bytes[..4].copy_from_slice(&self.x.to_ne_bytes());
        bytes[4..8].copy_from_slice(&self.y.to_ne_bytes());
        bytes[8..].copy_from_slice(&self.z.to_ne_bytes());
        bytes
    }

    pub fn distance_squared(&self, other: &Self) -> f32 {
        (other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2)
    }

    pub fn midpoint(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
            z: (self.z + other.z) / 2.0,
        }
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        *self * (1.0 - t) + *other * t
    }

}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Vector3 {
        vector * self
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Div for Vector3 {
    type Output = Vector3;

    fn div(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}