use glam::DVec3;
use indicatif::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use raytracing_in_a_wekeend_rust::camera::Camera;
use raytracing_in_a_wekeend_rust::hitable::{Hitable, HitableList, Sphere};
use raytracing_in_a_wekeend_rust::material::Material;
use raytracing_in_a_wekeend_rust::ray::Ray;
use std::time::Instant;

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
            DVec3::new(1.0, 1.0, 1.0) * (1.0 - t) + DVec3::new(0.50, 0.7, 0.90) * t
        }
    }
}

fn random_scene() -> HitableList {
    let material = Material::Lambertian {
        attenuation: DVec3::new(0.5, 0.5, 0.5),
    };
    let material_metal = Material::Metal {
        attenuation: DVec3::new(0.9, 0.9, 0.9),
        fuzziness: (0.1),
    };
    let material_refract = Material::Dielectric { refraction: 1.9 };
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    //let attenuation = DVec3::new(0.50, 0.5, 0.50);
    list.push(Box::new(Sphere::new(
        DVec3::new(10.0, 4.0, -100.0),
        4.0,
        material_refract.clone(),
    )));
    list.push(Box::new(Sphere::new(
        DVec3::new(-10.0, 4.0, -100.0),
        4.0,
        material.clone(),
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
    let start = Instant::now();
    //channels
    let channels = 3;

    //rendersettings
    let ray_per_pixel = 100;

    //scene
    let world_scene = random_scene();

    //Image
    const IMAGE_WIDTH: f64 = 512.0;
    const IMAGE_HEIGHT: f64 = 256.0;
    let pb = ProgressBar::new(IMAGE_WIDTH as u64 * IMAGE_HEIGHT as u64 * ray_per_pixel.clone());

    // Camera
    let lookfrom = DVec3::new(0.0, 5.0, 0.0);
    let lookat = DVec3::new(0.0, 0.0, -100.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.2;
    let camera = Camera::new(
        lookfrom,
        lookat,
        DVec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(IMAGE_WIDTH) / f64::from(IMAGE_HEIGHT),
        aperture,
        dist_to_focus,
    );

    let mut buffer = vec![0u8; (IMAGE_WIDTH as u32 * IMAGE_HEIGHT as u32 * channels) as usize];

    buffer
        .par_chunks_mut((IMAGE_WIDTH as u32 * channels) as usize)
        .rev()
        .enumerate()
        .for_each(|(j, row)| {
            let mut rng = rand::thread_rng();
            for (i, rgb) in row.chunks_mut(channels as usize).enumerate() {
                let mut pixel_colour = DVec3::new(0.0, 0.0, 0.0);
                for _ in 0..ray_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WIDTH as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT as f64;
                    let r = &camera.get_ray(u, v);
                    pixel_colour = pixel_colour + ray_color(&r, &world_scene, 0);
                    pb.inc(1);
                }
                pixel_colour = pixel_colour / ray_per_pixel as f64;

                let mut iter = rgb.iter_mut();
                *iter.next().unwrap() = (255.999 * pixel_colour.x) as u8;
                *iter.next().unwrap() = (255.999 * pixel_colour.y) as u8;
                *iter.next().unwrap() = (255.999 * pixel_colour.z) as u8;
            }
        });

    image::save_buffer(
        "D:/rust/raytracing_in_a_wekeend_rust/src/renders/render.png",
        &buffer,
        IMAGE_WIDTH as u32,
        IMAGE_HEIGHT as u32,
        image::ColorType::Rgb8,
    )
    .expect("Failed to save output image");
    //println!("{:?} {:?}", pb.position(), pb.length());

    //pb.length();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
