mod sphere;
use sphere::Sphere;
mod ray;
use ray::Ray;

pub struct World {
    objects: Vec<Sphere>,
}

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<hitrecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }

        tmp_rec
    }
}
