extern crate rand;

use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use self::rand::Rng;
use vector4::Vector4;

/// A 3 axis vector of `f32` values.
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Returns a new `Vector3`.
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    /// Returns a `Vector3` with a length of 0.
    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Returns a `Vector3` of values (0, 1, 0).
    pub fn up() -> Vector3 {
        Vector3 { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Returns a `Vector3` of values (0, 0, 1).
    pub fn forward() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Returns a `Vector3` of values (1, 0, 0).
    pub fn left() -> Vector3 {
        Vector3 { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Returns the dot product of `v0` and `v1`.
    pub fn dot(v0: Vector3, v1: Vector3) -> f32 {
        v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
    }

    /// Returns the cross product of `v0` and `v1`.
    pub fn cross(v0: Vector3, v1: Vector3) -> Vector3 {
        Vector3 {x: (v0.y * v1.z) - (v0.z * v1.y),
                 y: (v0.z * v1.x) - (v0.x * v1.z),
                 z: (v0.x * v1.y) - (v0.y * v1.x)}
    }

    /// Returns the length of the vector before taking the square root.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a new normalized `Vector3` of the vector.
    pub fn normalized(&self) -> Vector3 {
        let l = self.length();
        Vector3::new(self.x / l, self.y / l, self.z / l)
    }

    /// Returns the barycentric coordinates for `point` inside a triangle defined by `v0`, `v1`, `v2`.
    pub fn barycentric(point: Vector3, v0: Vector3, v1: Vector3, v2: Vector3) -> Option<Vector3> {
        let vec0 = v1 - v0;
        let vec1 = v2 - v0;
        let vec2 = point - v0;
        let d00 = Vector3::dot(vec0, vec0);
        let d01 = Vector3::dot(vec0, vec1);
        let d11 = Vector3::dot(vec1, vec1);
        let d20 = Vector3::dot(vec2, vec0);
        let d21 = Vector3::dot(vec2, vec1);
        let denom = d00 * d11 - d01 * d01;

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        Some(Vector3::new(u, v, w))
    }

    pub fn r(&self) -> f32 {
        self.x
    }

    pub fn g(&self) -> f32 {
        self.y
    }

    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        let mut rng = rand::thread_rng();
        let mut p = Vector3::zero();
        loop {
            p = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) * 2.0 - Vector3::new(1.0, 1.0, 1.0);

            if p.length_squared() < 1.0 {
                break;
            }
        }

        p
    }

    pub fn random_in_unit_disk() -> Vector3 {
        let mut rng = rand::thread_rng();
        let mut p = Vector3::zero();
        loop {
            p = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) * 2.0 - Vector3::new(1.0, 1.0, 0.0);

            if Vector3::dot(p, p) < 1.0 {
                break;
            }
        }

        p
    }

    pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
        v - n * 2.0 * Vector3::dot(v, n)
    }

    pub fn to_vector4(&self, w: f32) -> Vector4 {
        Vector4::new(self.x, self.y, self.z, w)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x,
                 y: self.y + other.y,
                 z: self.z + other.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x,
                 y: self.y - other.y,
                 z: self.z - other.z}
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {x: self.x * other,
                 y: self.y * other,
                 z: self.z * other}
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x * other.x,
                 y: self.y * other.y,
                 z: self.z * other.z}
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {x: self * other.x,
                 y: self * other.y,
                 z: self * other.z}
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f32) -> Vector3 {
        Vector3 {x: self.x / other,
                 y: self.y / other,
                 z: self.z / other}
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid Vector3 Index"),
        }
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
         match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid Vector3 Index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use vector3::Vector3;

    #[test]
    fn test_vector_subtraction() {
        let a = Vector3 {x: 1.0, y: 1.0, z: 1.0};
        let b = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let c = a - b;
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, -1.0);
        assert_eq!(c.z, -2.0);

        let d = a - b;
        assert_eq!(d.x, 0.0);
    }

    #[test]
    fn test_vector_cross() {
        let x = Vector3 {x: 1.0, y: 0.0, z: 0.0};
        let y = Vector3 {x: 0.0, y: 1.0, z: 0.0};
        let z = Vector3::cross(x, y);

        assert_eq!(0.0, z.x);
        assert_eq!(0.0, z.y);
        assert_eq!(1.0, z.z);
    }
}