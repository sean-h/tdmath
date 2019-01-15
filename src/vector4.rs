use super::Vector3;

/// A 4 axis vector of `f32` values.
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    /// Returns a new Vector4.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x,
            y,
            z,
            w,
        }
    }

    /// Returns a Vector4 with all values set to 0.
    pub fn zero() -> Vector4 {
        Vector4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Returns the `x`, `y` and `z` values as a `Vector3`.
    pub fn xyz(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}