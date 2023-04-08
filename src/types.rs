use crate::angles::angle2::Angle2;
use crate::angles::quaternion::Quaternion;
use crate::rendering::vertex::Vertex;
use crate::vectors::vector2::Vector2;
use crate::vectors::vector3::Vector3;

pub enum Axis {
    X,
    Y,
    Z,
}

pub type Point3 = (Quaternion, Vector3);
pub type Point2 = (Angle2, Vector2);

pub type Vertices = Vec<Vertex>;
