use crate::ray::Ray;
use glam::DVec3;
use rand::prelude::*;

fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::thread_rng();
    let mut p = DVec3::new(1.0, 1.0, 1.0);
    while p.length_squared() >= 1.0 {
        p = DVec3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) * 2.0
            - DVec3::new(1.0, 1.0, 1.0);
    }
    p
}
#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { attenuation: DVec3 },
    Metal { attenuation: DVec3, fuzziness: f64 },
}
fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - n * v.dot(n) * 2.0
}

impl Material {
    pub fn scatter(&self, r: &Ray, n: DVec3, p: DVec3) -> (Ray, DVec3, bool) {
        let target = p + n + random_in_unit_sphere();
        match self {
            Material::Lambertian { attenuation } => (Ray::new(p, target - p), *attenuation, true),
            Material::Metal {
                attenuation,
                fuzziness,
            } => {
                let reflected = reflect(r.direction().normalize(), n);
                let scattered = Ray::new(p, reflected + random_in_unit_sphere() * *fuzziness);
                let b = scattered.direction().dot(n) >= 0.0;
                (scattered, *attenuation, b)
            }
        }
    }
}
