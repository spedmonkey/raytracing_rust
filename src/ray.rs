use glam::DVec3;
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> DVec3 {
        self.origin
    }

    pub fn direction(&self) -> DVec3 {
        self.direction
    }

    pub fn point_at_parameter(&self, t: f64) -> DVec3 {
        self.origin() + self.direction() * t
    }
}
