use std::ops::{Add, Div, Mul, Sub};
use crate::math::fast_inv_sqrt;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {

    /// Creates a new vector with the given x, y, and z components.
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    /// Creates a new vector with all components set to 0.
    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }

    /// Creates a new vector with all components set to 1.
    pub fn one() -> Vector3 {
        Vector3::new(1.0, 1.0, 1.0)
    }

    /// Creates a new vector with all components set to the given value.
    pub fn from_one(x: f32) -> Vector3 {
        Vector3::new(x, x, x)
    }

    /// Returns the dot product of this and other vector.
    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the magnitude (length) of the vector.
    pub fn magnitude(&self) -> f32 {
        fast_inv_sqrt(self.magnitude_squared())
    }

    /// Returns the squared magnitude of this vector.
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns a normalized copy of this vector.
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

    pub fn reflect(self, normal: Vector3) -> Self {
        let dot = &self.dot(&normal) * 2.0;
        let mut result = normal.scale(dot);
        result = result - self;
        result
    }

    pub fn project(&self, other: Vector3) -> Self {
        other.scale(self.dot(&other) /  other.magnitude_squared())
    }

    pub fn scale(&self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
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

impl Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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