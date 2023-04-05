use crate::rotations::quaternion::Quaternion;

/// A Euler Angle representing a rotation around the X, Y, and Z axes.
/// This is just like Quaternion, but less complex.
#[derive(Debug, Clone, Copy)]
pub struct Euler {
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
}

impl Euler {

    /// Creates a new Euler angle struct with the given pitch, yaw, and roll values in radians.
    #[inline]
    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
        Self { pitch, yaw, roll }
    }

    /// Creates a new Euler angle struct with pitch, yaw, and roll set to 0.
    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn from_quaternion(q: &Quaternion) -> Self {
        let qw = q.w;
        let qx = q.x;
        let qy = q.y;
        let qz = q.z;

        let mut sinr_cosp = 2.0 * (qw * qx + qy * qz);
        let mut cosr_cosp = 1.0 - 2.0 * (qx * qx + qy * qy);

        let mut sinp = 2.0 * (qw * qy - qz * qx);
        let mut pitch: f32;
        if sinp.abs() >= 1.0 {
            pitch = (std::f32::consts::PI / 2.0) * sinp.signum();
        } else {
            pitch = sinp.asin();
        }

        let mut siny_cosp = 2.0 * (qw * qz + qx * qy);
        let mut cosy_cosp = 1.0 - 2.0 * (qy * qy + qz * qz);

        Self {
            roll: sinr_cosp.atan2(cosr_cosp),
            pitch,
            yaw: siny_cosp.atan2(cosy_cosp),
        }
    }

    fn to_quaternion(&self) -> Quaternion {
        let half_pitch = self.pitch * 0.5;
        let half_yaw = self.yaw * 0.5;
        let half_roll = self.roll * 0.5;

        let sin_pitch = half_pitch.sin();
        let cos_pitch = half_pitch.cos();
        let sin_yaw = half_yaw.sin();
        let cos_yaw = half_yaw.cos();
        let sin_roll = half_roll.sin();
        let cos_roll = half_roll.cos();

        Quaternion {
            w: cos_pitch * cos_yaw * cos_roll + sin_pitch * sin_yaw * sin_roll,
            x: sin_pitch * cos_yaw * cos_roll - cos_pitch * sin_yaw * sin_roll,
            y: cos_pitch * sin_yaw * cos_roll + sin_pitch * cos_yaw * sin_roll,
            z: cos_pitch * cos_yaw * sin_roll - sin_pitch * sin_yaw * cos_roll,
        }
    }
}
