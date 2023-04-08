use std::f32::consts::PI;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};
use crate::vectors::vector3::Vector3;

/// A 4x4 matrix with 16 `f32` elements stored in column-major order.
#[derive(Clone, Copy)]
pub struct Matrix4x4 {
    pub data: [f32; 16],
}

impl Matrix4x4 {

    /// Creates a new identity Matrix.
    /// This is basically just `Matrix::identity()`.
    pub fn new() -> Self {
        Matrix4x4::identity()
    }

    /// Creates a new identity matrix.
    ///
    /// An identity matrix is a matrix in which all the elements of the main diagonal are 1, and all other elements are 0.
    pub fn identity() -> Self {
        Matrix4x4 {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Creates a new translation matrix.
    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Matrix4x4 {
            data: [
                1.0, 0.0, 0.0, x,
                0.0, 1.0, 0.0, y,
                0.0, 0.0, 1.0, z,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Creates a new scaling matrix.
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Matrix4x4 {
            data: [
                x, 0.0, 0.0, 0.0,
                0.0, y, 0.0, 0.0,
                0.0, 0.0, z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    /// Create a new Matrix from a float array.
    pub fn from_array(data: [f32; 16]) -> Self {
        Matrix4x4 {
            data
        }
    }

    /// Transposes the matrix in-place.
    pub fn transpose(&mut self) {
        let mut temp: f32;
        for i in 0..4 {
            for j in (i + 1)..4 {
                temp = self.data[i * 4 + j];
                self.data[i * 4 + j] = self.data[j * 4 + i];
                self.data[j * 4 + i] = temp;
            }
        }
    }

    /// Calculates the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        let m11 = self.data[0];
        let m12 = self.data[1];
        let m13 = self.data[2];
        let m14 = self.data[3];
        let m21 = self.data[4];
        let m22 = self.data[5];
        let m23 = self.data[6];
        let m24 = self.data[7];
        let m31 = self.data[8];
        let m32 = self.data[9];
        let m33 = self.data[10];
        let m34 = self.data[11];
        let m41 = self.data[12];
        let m42 = self.data[13];
        let m43 = self.data[14];
        let m44 = self.data[15];

        let a = m22 * (m33 * m44 - m34 * m43) - m23 * (m32 * m44 - m34 * m42) + m24 * (m32 * m43 - m33 * m42);
        let b = m21 * (m33 * m44 - m34 * m43) - m23 * (m31 * m44 - m34 * m41) + m24 * (m31 * m43 - m33 * m41);
        let c = m21 * (m32 * m44 - m34 * m42) - m22 * (m31 * m44 - m34 * m41) + m24 * (m31 * m42 - m32 * m41);
        let d = m21 * (m32 * m43 - m33 * m42) - m22 * (m31 * m43 - m33 * m41) + m23 * (m31 * m42 - m32 * m41);

        m11 * a - m12 * b + m13 * c - m14 * d
    }

    /// Calculates the inverse of the matrix.
    pub fn inverse(&self) -> Option<Matrix4x4> {
        let mut result = Matrix4x4::new();

        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        let e = self[4];
        let f = self[5];
        let g = self[6];
        let h = self[7];
        let i = self[8];
        let j = self[9];
        let k = self[10];
        let l = self[11];
        let m = self[12];
        let n = self[13];
        let o = self[14];
        let p = self[15];

        let q = a * f - b * e;
        let r = a * g - c * e;
        let s = a * h - d * e;
        let t = b * g - c * f;
        let u = b * h - d * f;
        let v = c * h - d * g;
        let w = i * n - j * m;
        let x = i * o - k * m;
        let y = i * p - l * m;
        let z = j * o - k * n;
        let aa = j * p - l * n;
        let bb = k * p - l * o;

        let det = q * bb - r * aa + s * z + t * y - u * x + v * w;

        if det == 0.0 {
            return None;
        }

        let inv_det = 1.0 / det;

        result[0] = (f * bb - g * aa + h * z) * inv_det;
        result[1] = (-b * bb + c * aa - d * z) * inv_det;
        result[2] = (n * v - o * u + p * t) * inv_det;
        result[3] = (-j * v + k * u - l * t) * inv_det;
        result[4] = (-e * bb + g * y - h * x) * inv_det;
        result[5] = (a * bb - c * y + d * x) * inv_det;
        result[6] = (-m * v + o * s - p * r) * inv_det;
        result[7] = (i * v - k * s + l * r) * inv_det;
        result[8] = (e * aa - f * y + h * w) * inv_det;
        result[9] = (-a * aa + b * y - d * w) * inv_det;
        result[10] = (m * u - n * s + p * q) * inv_det;
        result[11] = (-i * u + j * s - l * q) * inv_det;
        result[12] = (-e * z + f * x - g * w) * inv_det;
        result[13] = (a * z - b * x + c * w) * inv_det;
        result[14] = (-m * t + n * r - o * q) * inv_det;
        result[15] = (i * t - j * r + k * q) * inv_det;

        Some(result)
    }

    pub fn rotate(&mut self, angle: f32, axis: Vector3) {
        let rad = angle * PI / 180.0;
        let cos = rad.cos();
        let sin = rad.sin();
        let one_minus_cos = 1.0 - cos;

        let x = axis.x;
        let y = axis.y;
        let z = axis.z;

        let mut r = Matrix4x4::new();
        r[0] = cos + x * x * one_minus_cos;
        r[1] = x * y * one_minus_cos + z * sin;
        r[2] = x * z * one_minus_cos - y * sin;
        r[4] = x * y * one_minus_cos - z * sin;
        r[5] = cos + y * y * one_minus_cos;
        r[6] = y * z * one_minus_cos + x * sin;
        r[8] = x * z * one_minus_cos + y * sin;
        r[9] = y * z * one_minus_cos - x * sin;
        r[10] = cos + z * z * one_minus_cos;

        *self = *self * r;
    }

}

impl Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..16 {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Sub<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..16 {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in (0..16).step_by(4) {
            let a = self[i];
            let b = self[i + 1];
            let c = self[i + 2];
            let d = self[i + 3];
            result[i] = a * other[0] + b * other[4] + c * other[8] + d * other[12];
            result[i + 1] = a * other[1] + b * other[5] + c * other[9] + d * other[13];
            result[i + 2] = a * other[2] + b * other[6] + c * other[10] + d * other[14];
            result[i + 3] = a * other[3] + b * other[7] + c * other[11] + d * other[15];
        }
        result
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, scalar: f32) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        for i in 0..16 {
            result[i] = self[i] * scalar;
        }
        result
    }
}

impl Div<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    fn div(self, scalar: f32) -> Matrix4x4 {
        let mut result = Matrix4x4::new();
        let inv_scalar = 1.0 / scalar;
        for i in 0..16 {
            result[i] = self[i] * inv_scalar;
        }
        result
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}