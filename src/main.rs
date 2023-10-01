use glam::DVec3;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use itertools::{self, iproduct, Itertools};
use rand::{random, Rng};
use raytracing_in_a_wekeend_rust::camera::Camera;
use raytracing_in_a_wekeend_rust::colour::Colour;
use raytracing_in_a_wekeend_rust::hitable::{Hitable, HitableList, Sphere};
use raytracing_in_a_wekeend_rust::material::Material;
use raytracing_in_a_wekeend_rust::material::Material::Lambertian;
use raytracing_in_a_wekeend_rust::ray::Ray;
use std::ops::{Div, Mul};

fn ray_color(r: &Ray, world: &dyn Hitable, depth: u32) -> DVec3 {
    match world.hit(r, 0.0001, std::f64::INFINITY) {
        Some((hit_record, material)) => {
            let n = hit_record.normal();
            let p = hit_record.p();
            let (scattered, attenuation, b) = material.scatter(r, n, p);
            if depth < 32 && b {
                return attenuation * ray_color(&scattered, world, depth + 1);
            } else {
                return DVec3::new(0.0, 0.0, 0.0);
            }
        }
        None => {
            let unit_direction = r.direction().normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            DVec3::new(1.0, 1.0, 1.0) * (1.0 - t) + DVec3::new(1.0, 0.7, 0.50) * t
        }
    }
}

fn random_scene() -> HitableList {
    let material = Material::Lambertian {
        attenuation: DVec3::new(0.5, 0.5, 0.5),
    };
    let material_metal = Material::Metal {
        attenuation: DVec3::new(0.2, 0.2, 0.2),
        fuzziness: (0.5),
    };
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    //let attenuation = DVec3::new(0.50, 0.5, 0.50);
    list.push(Box::new(Sphere::new(
        DVec3::new(10.0, 4.0, -100.0),
        4.0,
        material_metal.clone(),
    )));
    list.push(Box::new(Sphere::new(
        DVec3::new(0.0, 8.0, -100.0),
        8.0,
        material_metal.clone(),
    )));
    list.push(Box::new(Sphere::new(
        DVec3::new(0.0, -10000.0, -100.0),
        10000.10,
        material.clone(),
    )));

    let hitable_list = HitableList::new(list);
    return hitable_list;
}

fn main() {
    //rendersettings
    let ray_per_pixel = 100;

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
    let lookfrom = DVec3::new(0.0, 5.0, 0.0);
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
    let mut rng = rand::thread_rng();
    for (index, pixel) in pixels.enumerate() {
        let mut pixel_colour = DVec3::new(0.0, 0.0, 0.0);
        let my_vec = vec![-0.66, -0.33, 0.33, 0.66];
        let my_vec = vec![0.0];
        let samples = iproduct!(my_vec.clone(), my_vec.clone());

        for _ in 0..ray_per_pixel {
            let u = (f64::from(pixel.0) + rng.gen::<f64>()) / f64::from(image_width);
            let v = (f64::from(pixel.1) + rng.gen::<f64>()) / f64::from(image_height);
            let r = &camera.get_ray(u, v);
            pixel_colour = pixel_colour + ray_color(&r, &world_scene, 0);
        }
        pixel_colour = pixel_colour / f64::from(ray_per_pixel);

        //let r = &camera.get_ray(pixel.0 as f64 / image_width, pixel.1 as f64 / image_height);
        //pixel_colour = pixel_colour + ray_color(&r, &world_scene, 0);

        let r = pixel.0 as f64 / (image_width) as f64;
        let g = pixel.1 as f64 / (image_height) as f64;
        let b = 0.0;

        let ir = (255.999 * pixel_colour.x) as u8;
        let ig = (255.999 * pixel_colour.y) as u8;
        let ib = (255.999 * pixel_colour.z) as u8;

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
