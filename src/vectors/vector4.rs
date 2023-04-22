use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::angles::quaternion::Quaternion;
use crate::math::fast_inv_sqrt;

/// A vector with x, y, z and w components.
/// They are used to represent a point or direction in 4d space.
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4 {

    /// Creates a new vector with the given x, y, z, and w components.
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4 { x, y, z, w }
    }

    /// Creates a new vector with all components set to 0.
    #[inline]
    pub fn zero() -> Self {
        Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    /// Creates a new vector with all components set to 1.
    #[inline]
    pub fn one() -> Self {
        Vector4 { x: 1.0, y: 1.0, z: 1.0, w: 1.0 }
    }

    /// Returns the dot product of this and other vector.
    #[inline]
    pub fn dot(self, other: &Vector4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Creates a new vector with all components set to the given value.
    #[inline]
    pub fn from_one(one: f32) -> Self {
        Vector4::new(one, one, one, one)
    }

    /// Returns the cross product of this and other vector.
    #[inline]
    pub fn cross(self, other: Vector4) -> Self {
        Vector4::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0,
        )
    }

    /// Returns the magnitude (length) of the vector.
    #[inline]
    pub fn magnitude(&self) -> f32 {
        fast_inv_sqrt(self.squared_magnitude())
    }

    /// Returns the squared magnitude of this vector.
    #[inline]
    pub fn squared_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns a normalized copy of this vector.
    #[inline]
    pub fn normalized(&mut self) -> Self {
        let inv_mag = fast_inv_sqrt(self.squared_magnitude());
        self.scale(inv_mag)
    }

    /// Scales this vector by the given factor.
    pub fn scale(&mut self, factor: f32) -> Self {
        let mut copy = self.clone();
        copy.x *= factor;
        copy.y *= factor;
        copy.z *= factor;
        copy.w *= factor;
        copy
    }

    /// Converts this vector to a quaternion
    #[inline]
    pub fn to_quaternion(&self) -> Quaternion {
        Quaternion::new(self.x, self.y, self.z, self.w)
    }

    /// Reflects the vector around the given normal.
    #[inline]
    pub fn reflect(&self, normal: &mut Vector4) -> Vector4 {
        *self - normal.scale(2.0 * self.dot(normal))
    }

    /// Projects the vector onto the given vector.
    pub fn project(&self, other: &mut Vector4) -> Vector4 {
        let dot_product = self.dot(other);
        let other_squared_magnitude = other.squared_magnitude();
        let scale_factor = dot_product / other_squared_magnitude;
        other.scale(scale_factor)
    }

    /// Returns the middle of this vector and the given vector.
    pub fn middle(&self, other: &Self) -> Self {
        let x = (self.x + other.x) * 0.5;
        let y = (self.y + other.y) * 0.5;
        let z = (self.z + other.z) * 0.5;
        let w = (self.w + other.w) * 0.5;
        Vector4 { x, y, z, w }
    }

    /// Converts the Vectors components to a byte array
    pub fn as_bytes(&self) -> &[u8; 16] {
        unsafe { &*(self as *const Self as *const [u8; 16]) }
    }

    /// Computes the squared distance between two vectors
    fn distance_squared(self, other: Vector4) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dw = self.w - other.w;
        dx * dx + dy * dy + dz * dz + dw * dw
    }

    /// Performs a linear interpolation between two vectors
    #[inline]
    fn lerp(self, other: Vector4, t: f32) -> Vector4 {
        let one_minus_t = 1.0 - t;
        Vector4 {
            x: self.x * one_minus_t + other.x * t,
            y: self.y * one_minus_t + other.y * t,
            z: self.z * one_minus_t + other.z * t,
            w: self.w * one_minus_t + other.w * t,
        }
    }
}

impl Neg for Vector4 {
    type Output = Self;

    fn neg(self) -> Self {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f32::EPSILON
            && (self.y - other.y).abs() < f32::EPSILON
            && (self.z - other.z).abs() < f32::EPSILON
            && (self.w - other.w).abs() < f32::EPSILON
    }
}

// Overloading the '+' operator for adding two vectors
impl Add for Vector4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

// Overloading the '-' operator for subtracting two vectors
impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

// Overloading the '*' operator for scalar multiplication
impl Mul<f32> for Vector4 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
    }
}

// Overloading the '*' operator for dot product of two vectors
impl Mul for Vector4 {
    type Output = f32;

    fn mul(self, other: Self) -> f32 {
        self.dot(&other)
    }
}

// Overloading the '/' operator for scalar division
impl Div<f32> for Vector4 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }
}