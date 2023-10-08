use crate::ray::Ray;
use glam::DVec3;
use std::f64::consts::PI;

use rand::prelude::*;

pub fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = DVec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length() < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: DVec3,
        lookat: DVec3,
        vup: DVec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = lookfrom;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let lower_left_corner =
            origin - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * half_width * focus_dist * 2.0;
        let vertical = v * half_height * focus_dist * 2.0;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    fn origin(&self) -> DVec3 {
        self.origin
    }

    fn horizontal(&self) -> DVec3 {
        self.horizontal
    }

    fn vertical(&self) -> DVec3 {
        self.vertical
    }

    fn lower_left_corner(&self) -> DVec3 {
        self.lower_left_corner
    }

    fn u(&self) -> DVec3 {
        self.u
    }

    fn v(&self) -> DVec3 {
        self.v
    }

    fn lens_radius(&self) -> f64 {
        self.lens_radius
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius();
        let offset = self.u() * rd.x + self.v() * rd.y;
        Ray::new(
            self.origin() + offset,
            self.lower_left_corner() + s * self.horizontal() + t * self.vertical()
                - self.origin()
                - offset,
        )
    }
}
