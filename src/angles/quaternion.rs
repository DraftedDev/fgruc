use std::f32::consts::FRAC_PI_2;
use std::ops::{Add, Mul, Sub};
use crate::math::{fast_inv_sqrt, fast_sin};

/// A 3D quaternion with scalar and vector components.
/// Used to represent angles in 3D space.
///
/// Only use if you really know Quaternions inside out.
/// Maybe use `Euler` struct instead.
///
/// NOTE: Some transformation functions are implemented in `Euler`, so you may need to use `Quaternion::to_euler()`.
#[derive(Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Quaternion {

    /// Creates a new quaternion with the given scalar (w) and vector (x,y,z) components.
    #[inline]
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self { w, x, y, z }
    }

    /// Returns an identity Quaternion.
    /// The identity represents a rotation of zero degrees around the x, y, and z axes.
    /// It is is defined as (1, 0, 0, 0), meaning that it has a scalar part of 1 and a vector part of (0, 0, 0).
    #[inline]
    pub fn identity() -> Self {
        Quaternion::new(1.0, 0.0, 0.0, 0.0)
    }

    /// Returns the squared magnitude of the quaternion.
    #[inline]
    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Returns the magnitude of the quaternion.
    #[inline]
    pub fn magnitude(&self) -> f32 {
        fast_inv_sqrt(self.magnitude_squared())
    }

    /// Returns the normalized version of the quaternion.
    pub fn normalized(&self) -> Quaternion {
        let mag = self.magnitude();
        Quaternion {
            x: self.x * mag,
            y: self.y * mag,
            z: self.z * mag,
            w: self.w * mag,
        }
    }

    /// Returns the inverse of the quaternion.
    pub fn inverse(&self) -> Quaternion {
        let magnitude_squared = self.magnitude_squared();
        Quaternion {
            x: -self.x / magnitude_squared,
            y: -self.y / magnitude_squared,
            z: -self.z / magnitude_squared,
            w: self.w / magnitude_squared,
        }
    }

    /// Returns the conjugate of the quaternion.
    #[inline]
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    /// Returns a new Quaternion that is a linear interpolation between `self` and `other` by `t`.
    /// `t` should be in the range [0, 1].
    pub fn lerp(&self, other: Quaternion, t: f32) -> Quaternion {
        let minus_t = 1.0 - t;
        Quaternion {
            x: minus_t * self.x + t * other.x,
            y: minus_t * self.y + t * other.y,
            z: minus_t * self.z + t * other.z,
            w: minus_t * self.w + t * other.w,
        }
    }

    /// Returns a new Quaternion that is a spherical linear interpolation between `self` and `other` by `t`.
    /// `t` should be in the range [0, 1].
    pub fn slerp(&self, other: Quaternion, t: f32) -> Quaternion {
        let cos_theta = self.dot(&other);
        let angle = cos_theta.acos();
        let sin_theta = fast_sin(angle);

        if sin_theta < 0.001 {
            // Linear interpolation if angle is small
            self.lerp(other, t)
        } else {
            let self_coeff = (1.0 - t) * fast_sin(angle);
            let other_coeff = t * fast_sin(angle);
            Quaternion {
                x: self_coeff * self.x + other_coeff * other.x,
                y: self_coeff * self.y + other_coeff * other.y,
                z: self_coeff * self.z + other_coeff * other.z,
                w: self_coeff * self.w + other_coeff * other.w,
            }.normalized()
        }
    }

    /// Returns the dot product of this and the other quaternion.
    #[inline]
    pub fn dot(&self, other: &Quaternion) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Creates a new quaternion from the given euler angles.
    pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Self {
        let (sp, cp) = (pitch * 0.5).sin_cos();
        let (sy, cy) = (yaw * 0.5).sin_cos();
        let (sr, cr) = (roll * 0.5).sin_cos();

        let w = cr * cp * cy + sr * sp * sy;
        let x = sr * cp * cy - cr * sp * sy;
        let y = cr * sp * cy + sr * cp * sy;
        let z = cr * cp * sy - sr * sp * cy;

        Self { x, y, z, w }
    }

    /// Converts this quaternion to euler angles.
    pub fn to_euler(&self) -> (f32, f32, f32) {
        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let roll = sinr_cosp.atan2(cosr_cosp);

        let sinp = 2.0 * (self.w * self.y - self.z * self.x);
        let pitch = if sinp.abs() >= 1.0 {
            FRAC_PI_2.copysign(sinp)
        } else {
            sinp.asin()
        };

        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let yaw = siny_cosp.atan2(cosy_cosp);

        (pitch, yaw, roll)
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w + rhs.w,
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
        )
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        )
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
