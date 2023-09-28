use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use itertools::{self, iproduct, Itertools};
use rand::{random, Rng};
use raytracing_in_a_wekeend_rust::camera::Camera;
use raytracing_in_a_wekeend_rust::colour::Colour;
use raytracing_in_a_wekeend_rust::hitable;
use raytracing_in_a_wekeend_rust::hitable::{Hitable, HitableList, Sphere};
use raytracing_in_a_wekeend_rust::ray::Ray;
use std::ops::{Div, Mul};
//some other test

fn hit_sphere(center: DVec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = DVec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * DVec3::dot(oc, ray.direction());
    let c = DVec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    };
}

fn ray_color(r: &Ray, world: &dyn Hitable) -> Colour {
    //let t = hit_sphere(DVec3::new(0.0, 0.0, -1.0), 0.5, r);
    let depth = 2;
    match world.hit(r, 0.001, std::f64::INFINITY) {
        Some(hit_record) => {
            let n = hit_record.normal();
            let p = hit_record.p();
            let colour = DVec3::new(0.0, 0.0, 0.0);
            let colour = n;
            return Colour::new(colour.x, colour.y, colour.z);
        }
        None => {
            let unit_direction = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            let colour = DVec3::new(1.0, 1.0, 1.0) * (1.0 - t) + DVec3::new(0.5, 0.7, 1.0) * t;
            return Colour::new(colour.x, colour.y, colour.z);
        }
    }

    /*
    let unit_direction = r.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let divider = 1.7;
    let grad = DVec3::new(a / divider, a / divider, a / divider);
    let color_mult = DVec3::new(0.5 / divider, 0.7 / divider, 1.0 / divider);
    let result = grad + color_mult;
    result
     */
    //let unit_direction = r.direction().normalize();
    //let a = 0.5 * (1.0 + unit_direction.y);
    //return (Colour::new(1.0, 1.0, 1.0)) * (1.0 - a) + Colour::new(0.5, 0.7, 1.0) * a;
}

fn random_scene() -> HitableList {
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Sphere::new(DVec3::new(0.0, 0.0, -100.0), 10.0)));
    list.push(Box::new(Sphere::new(DVec3::new(15.0, 10.0, -100.0), 4.0)));
    HitableList::new(list)
}

fn main() {
    //scene
    let world_scene = random_scene();

    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const image_width: f64 = 512.0;
    const image_height: f64 = 256.0;
    let pb = ProgressBar::new(image_width as u64 * image_height as u64);
    let mut buffer: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);

    // Camera
    let focal_length = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera_centre = DVec3::new(0.0, 0.0, 0.0);
    let lookfrom = DVec3::new(0.0, 0.0, 0.0);
    let lookat = DVec3::new(0.0, 0.0, -100.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.2;
    let camera = Camera::new(
        lookfrom,
        lookat,
        DVec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(image_width) / f64::from(image_height),
        aperture,
        dist_to_focus,
    );
    let pixels = (0..image_width as u32).cartesian_product(0..image_height as u32);

    for (index, pixel) in pixels.enumerate() {
        let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
        let my_vec = vec![-0.66, -0.33, 0.33, 0.66];
        let my_vec = vec![0.0];
        let samples = iproduct!(my_vec.clone(), my_vec.clone());
        for (offset_x, offset_y) in samples.clone() {
            let r = &camera.get_ray(
                (pixel.0 as f64 / image_width) + offset_x / image_width,
                (pixel.1 as f64 / image_height) + offset_y / image_width,
            );
            pixel_colour = pixel_colour + ray_color(&r, &world_scene);
        }
        pixel_colour = pixel_colour / samples.count() as f64;

        let r = pixel.0 as f64 / (image_width) as f64;
        let g = pixel.1 as f64 / (image_height) as f64;
        let b = 0.0;

        let ir = (255.999 * pixel_colour.r) as u8;
        let ig = (255.999 * pixel_colour.g) as u8;
        let ib = (255.999 * pixel_colour.b) as u8;

        //*pixel = Rgb([ir, ig, ib]);
        buffer.put_pixel(
            pixel.0,
            (image_height - 1.0) as u32 - pixel.1,
            Rgb([ir, ig, ib]),
        );
        pb.inc(1);
    }

    buffer
        .save("D:/rust/raytracing_in_a_wekeend_rust/src/renders/render.png")
        .unwrap();
}
