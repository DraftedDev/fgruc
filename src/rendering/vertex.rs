use crate::rendering::color::UniColor;
use crate::vectors::vector2::Vector2;
use crate::vectors::vector3::Vector3;

/// A Vertex struct for representing a point in 3D space along with its associated attributes such as
/// normal, texture coordinates, color, etc.
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub tex_coords: Vector2,
    pub color: UniColor,
}

impl Vertex {

    /// Creates a new vertex from the given values
    pub fn new(position: Vector3, normal: Vector3, tex_coords: Vector2, color: UniColor) -> Self {
        Self {
            position,
            normal,
            tex_coords,
            color,
        }
    }

    /// Returns the byte representation of the vertex.
    pub fn as_bytes(&self) -> [u8; 40] {
        let mut bytes = [0u8; 40];
        bytes[..12].copy_from_slice(&self.position.as_bytes());
        bytes[12..24].copy_from_slice(&self.normal.as_bytes());
        bytes[24..32].copy_from_slice(&self.tex_coords.as_bytes());
        bytes[32..].copy_from_slice(&self.color.as_bytes());
        bytes
    }

    /// Linearly interpolate between two vertices
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            position: self.position.lerp(&other.position, t),
            normal: self.normal.lerp(&other.normal, t),
            tex_coords: self.tex_coords.lerp(&other.tex_coords, t),
            color: self.color.lerp(&other.color, t),
        }
    }

    /// Compute the midpoint between two vertices
    pub fn midpoint(&self, other: &Self) -> Self {
        Self {
            position: self.position.midpoint(&other.position),
            normal: self.normal.midpoint(&other.normal),
            tex_coords: self.tex_coords.midpoint(&other.tex_coords),
            color: self.color.midpoint(&other.color),
        }
    }

    /// Compute the squared Euclidean distance between two vertices
    pub fn distance_squared(&self, other: &Self) -> f32 {
        self.position.distance_squared(&other.position)
            + self.normal.distance_squared(&other.normal)
            + self.tex_coords.distance_squared(&other.tex_coords)
            + (self.color.distance_squared(&other.color) as f32)
    }

    /// Compute the Euclidean distance between two vertices
    pub fn distance(&self, other: &Self) -> f32 {
        self.distance_squared(other).sqrt()
    }

}
