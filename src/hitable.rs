//use crate::material::Material;
use crate::material::{self, Material};
use crate::ray::Ray;
use glam::DVec3;

pub struct HitRecord {
    t: f64,
    p: DVec3,
    normal: DVec3,
}

impl HitRecord {
    pub fn new(t: f64, p: DVec3, normal: DVec3) -> HitRecord {
        HitRecord { t, p, normal }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn p(&self) -> DVec3 {
        self.p
    }

    pub fn normal(&self) -> DVec3 {
        self.normal
    }
}

pub trait Hitable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
}

pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Material) -> Sphere {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> DVec3 {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let oc = r.origin() - self.center();
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius() * self.radius();
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 < t_max && t1 > t_min {
                let p = r.point_at_parameter(t1);
                let n = (p - self.center()) / self.radius();
                Some((HitRecord::new(t1, p, n), &self.material))
            } else if t2 < t_max && t2 > t_min {
                let p = r.point_at_parameter(t2);
                let n = (p - self.center()) / self.radius();
                Some((HitRecord::new(t2, p, n), &self.material))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let mut closest_so_far = t_max;
        let mut res = None;
        for h in self.list.iter() {
            if let Some((hit_record, material)) = h.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t();
                res = Some((
                    HitRecord::new(hit_record.t(), hit_record.p(), hit_record.normal()),
                    material,
                ))
            }
        }
        res
    }
}
