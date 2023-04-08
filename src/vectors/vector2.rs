use std::ops::{Add, Div, Mul, Sub};
use crate::math::fast_inv_sqrt;

/// A 2D vector for representing points or directions in 2D space.
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {

    /// Create a Vector2 with x and y components.
    #[inline]
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Create a Vector2 with both x and y set to 0.
    #[inline]
    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    /// Create a Vector2 with both x and y set to 1.
    #[inline]
    pub fn one() -> Vector2 {
        Vector2::new(1.0, 1.0)
    }

    /// Create a Vector2 with a single f32 as both x and y.
    #[inline]
    pub fn from_one(x: f32) -> Vector2 {
        Vector2::new(x,x)
    }

    /// Returns the dot product of this and other vector.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Returns a normalized version of the vector.
    #[inline]
    pub fn normalized(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> f32 {
        1.0 / fast_inv_sqrt(self.x * self.x + self.y * self.y)
    }

    /// Returns the squared magnitude (length) of the vector.
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Reflects the vector about the given normal.
    pub fn reflect(&self, normal: Vector2) -> Vector2 {
        let d = self.dot(normal);
        Vector2 {
            x: self.x - 2.0 * d * normal.x,
            y: self.y - 2.0 * d * normal.y,
        }
    }

    /// Projects the vector onto the vector other.
    #[inline]
    pub fn project(&self, other: Vector2) -> Vector2 {
        other * (self.dot(other) / other.magnitude_squared())
    }

    /// Scales the vector by the given scalar.
    #[inline]
    pub fn scale(&self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Returns the byte representation of the vector.
    pub fn as_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&self.x.to_ne_bytes());
        bytes[4..].copy_from_slice(&self.y.to_ne_bytes());
        bytes
    }

    /// Returns the squared distance between this and other Vector2.
    pub fn distance_squared(&self, other: &Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }

    /// Returns the midpoint between this and other Vector2.
    pub fn midpoint(&self, other: &Self) -> Self {
        Self::new((self.x + other.x) / 2.0, (self.y + other.y) / 2.0)
    }

    /// Returns the lerped version of this and other Vector2.
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::new(
            (1.0 - t) * self.x + t * other.x,
            (1.0 - t) * self.y + t * other.y,
        )
    }

}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}