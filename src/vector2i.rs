use vector3::Vector3;
use std::cmp::{min, max};
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};

/// A 2 axis vector of `i32` values.
#[derive(Debug, Copy, Clone)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    /// Returns a new `Vector2i`.
    pub fn new(x: i32, y: i32) -> Vector2i {
        Vector2i { x, y }
    }

    /// Returns the barycentric coordinates for `point` inside a triangle defined by `v0`, `v1`, `v2`.
    /// Returns `None` if the point is outside the triangle.
    pub fn barycentric(point: Vector2i, v0: Vector2i, v1: Vector2i, v2: Vector2i) -> Option<Vector3> {
        let x = Vector3::new((v2.x - v0.x) as f32, (v1.x - v0.x) as f32, (v0.x - point.x) as f32);
        let y = Vector3::new((v2.y - v0.y) as f32, (v1.y - v0.y) as f32, (v0.y - point.y) as f32);

        let u = Vector3::cross(x, y);

        if u.z < 1.0 {
            return None;
        }

        Some(Vector3::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z))
    }

    /// Returns the bounding box for the points `v0`, `v1`, `v2` as (Minimum, Maximum).
    pub fn bbox3(v0: Vector2i, v1: Vector2i, v2: Vector2i) -> (Vector2i, Vector2i) {
        let min_x = min(v0.x, min(v1.x, v2.x));
        let max_x = max(v0.x, max(v1.x, v2.x));
        let min_y = min(v0.y, min(v1.y, v2.y));
        let max_y = max(v0.y, max(v1.y, v2.y));

        (Vector2i::new(min_x, min_y),
         Vector2i::new(max_x, max_y))
    }
}

impl Add for Vector2i {
    type Output = Vector2i;

    fn add(self, other: Vector2i) -> Vector2i {
        Vector2i { x: self.x + other.x,
                   y: self.y + other.y }
    }
}

impl Sub for Vector2i {
    type Output = Vector2i;

    fn sub(self, other: Vector2i) -> Vector2i {
        Vector2i { x: self.x - other.x,
                   y: self.y - other.y }
    }
}

impl Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, other: i32) -> Vector2i {
        Vector2i { x: self.x * other,
                   y: self.y * other }
    }
}

impl Mul<Vector2i> for i32 {
    type Output = Vector2i;

    fn mul(self, other: Vector2i) -> Vector2i {
        Vector2i { x: self * other.x,
                   y: self * other.y }
    }
}

impl Mul<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn mul(self, other: Vector2i) -> Vector2i {
        Vector2i { x: self.x * other.x,
                   y: self.y * other.y }
    }
}

impl Div<i32> for Vector2i {
    type Output = Vector2i;

    fn div(self, other: i32) -> Vector2i {
        Vector2i { x: self.x / other,
                   y: self.y / other }
    }
}

impl Neg for Vector2i {
    type Output = Vector2i;

    fn neg(self) -> Vector2i {
        Vector2i { x: -self.x,
                   y: -self.y }
    }
}

impl Index<usize> for Vector2i {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Invalid Vector2i index"),
        }
    }
}

impl IndexMut<usize> for Vector2i {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Invalid Vector2i index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use vector2i::Vector2i;

    #[test]
    fn test_vector2i_add() {
        let a = Vector2i::new(0, 5);
        let b = Vector2i::new(-6, 4);
        let c = a + b;
        assert_eq!(c.x, -6);
        assert_eq!(c.y, 9);
    }

    #[test]
    fn test_vector2i_sub() {
        let a = Vector2i::new(0, 5);
        let b = Vector2i::new(-6, 4);
        let c = a - b;
        assert_eq!(c.x, 6);
        assert_eq!(c.y, 1);
    }

    #[test]
    fn test_vector2i_mul() {
        let a = Vector2i::new(0, 5);
        let b = 3;
        let c = a * b;
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 15);

        let c = b * a;
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 15);
    }

    #[test]
    fn test_vector2i_mul_vector2i() {
        let a = Vector2i::new(0, 5);
        let b = Vector2i::new(3, 2);
        let c = a * b;
        assert_eq!(c.x, 0);
        assert_eq!(c.y, 10);
    }

    #[test]
    fn test_vector2i_div() {
        let a = Vector2i::new(12, 5);
        let b = 3;
        let c = a / b;
        assert_eq!(c.x, 4);
        assert_eq!(c.y, 1);
    }

    #[test]
    fn test_vector2i_neg() {
        let a = Vector2i::new(12, 0);
        let c = -a;
        assert_eq!(c.x, -12);
        assert_eq!(c.y, 0);
    }

    #[test]
    fn test_vector2i_index() {
        let a = Vector2i::new(12, 0);
        assert_eq!(a[0], 12);
        assert_eq!(a[1], 0);
    }

    #[test]
    #[should_panic]
    fn test_vector2i_index_panic() {
        let a = Vector2i::new(12, 0);
        assert_eq!(a[2], 0);
    }
}