use super::hit::{Hit, HitRecord};
use glam::DVec3;
use raytracing_in_a_wekeend_rust::ray::Ray;

pub struct Box {
    center: DVec3,
    size: f64,
}

impl Box {
    pub fn new(center: DVec3, size: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Box {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
