use vector3::Vector3;

/// A 3D ray with an origin, direction and time.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector3,
    direction: Vector3,
    time: f32,
}

impl Ray {
    /// Returns a new `Ray`.
    pub fn new(origin: Vector3, direction: Vector3, time: f32) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    /// Returns the origin of the `Ray`.
    pub fn origin(&self) -> Vector3 {
        self.origin
    }

    /// Returns the direction of the `Ray`.
    pub fn direction(&self) -> Vector3 {
        self.direction
    }
    
    /// Returns the time of the `Ray`.
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Returns the point at time `t` along the `Ray`.
    pub fn point_at_parameter(&self, t: f32) -> Vector3 {
        self.origin + self.direction * t
    }
}