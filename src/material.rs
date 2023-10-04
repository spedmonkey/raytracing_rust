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
    Dielectric { refraction: f64 },
}
fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - n * v.dot(n) * 2.0
}

fn refract(v: DVec3, n: DVec3, ni_over_nt: f64) -> Option<DVec3> {
    let uv = v.normalize();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlik(cosine: f64, refraction: f64) -> f64 {
    let r0 = ((1.0 - refraction) / (1.0 + refraction)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

fn scatter_lambertian(target: DVec3, p: DVec3, attenuation: DVec3) -> (Ray, DVec3, bool) {
    return (Ray::new(p, target - p), attenuation, true);
}

fn scatter_metal(
    r: &Ray,
    n: DVec3,
    p: DVec3,
    attenuation: DVec3,
    fuzziness: f64,
) -> (Ray, DVec3, bool) {
    let reflected = reflect(r.direction().normalize(), n);
    let scattered = Ray::new(p, reflected + random_in_unit_sphere() * fuzziness);
    let b = scattered.direction().dot(n) >= 0.0;
    (scattered, attenuation, b)
}

impl Material {
    pub fn scatter(&self, r: &Ray, n: DVec3, p: DVec3) -> (Ray, DVec3, bool) {
        let target = p + n + random_in_unit_sphere();
        match self {
            Material::Lambertian { attenuation } => scatter_lambertian(target, p, *attenuation),
            Material::Metal {
                attenuation,
                fuzziness,
            } => scatter_metal(r, n, p, *attenuation, *fuzziness),
            Material::Dielectric { refraction } => {
                let reflected = reflect(r.direction(), n);
                let (outward_normal, ni_over_nt, cosine) = if r.direction().dot(n) > 0.0 {
                    (
                        -n,
                        *refraction,
                        refraction * r.direction().dot(n) / r.direction().length(),
                    )
                } else {
                    (
                        n,
                        1.0 / refraction,
                        -(r.direction().dot(n)) / r.direction().length(),
                    )
                };
                let scattered = match refract(r.direction(), outward_normal, ni_over_nt) {
                    Some(refracted) => {
                        let reflect_prob = schlik(cosine, *refraction);
                        let mut rng = rand::thread_rng();
                        if rng.gen::<f64>() < reflect_prob {
                            Ray::new(p, reflected)
                        } else {
                            Ray::new(p, refracted)
                        }
                    }
                    None => Ray::new(p, reflected),
                };
                let attenuation = DVec3::new(1.0, 1.0, 1.0);
                (scattered, attenuation, true)
            }
        }
    }
}
