#![deny(clippy::all)]
#![forbid(unsafe_code)]
use glam::DVec3;
use image::buffer;
use indicatif::ProgressBar;
use rand::Rng;

use rand_pcg::Pcg32;
use rayon::prelude::*;
use raytracing_in_a_wekeend_rust::camera::Camera;
use raytracing_in_a_wekeend_rust::hitable::{Hitable, HitableList, Sphere};
use raytracing_in_a_wekeend_rust::material::Material;
use raytracing_in_a_wekeend_rust::ray::Ray;
use seeded_random::{Random, Seed};
use std::process;

//use pixels::{wgpu, PixelsContext};

use std::time::Instant;

mod gui;
use crate::gui::Framework;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, PixelsBuilder, PixelsContext, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
const WIDTH: u32 = 512;
const HEIGHT: u32 = 256;
const BOX_SIZE: i16 = 64;

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn ray_color(r: &Ray, world: &dyn Hitable, depth: u32) -> DVec3 {
    match world.hit(r, 0.0001, std::f64::INFINITY) {
        Some((hit_record, material)) => {
            let n = hit_record.normal();
            let p = hit_record.p();
            let (scattered, attenuation, b) = material.scatter(r, n, p);
            if depth < 32 && b {
                return attenuation * ray_color(&scattered, world, depth + 1);
            } else {
                return material.get_emission();
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
    let material = Material::Lambertian {
        attenuation: DVec3::new(0.5, 0.5, 0.5),
    };
    let material_sun: Material = Material::Light {
        emission: DVec3::new(1.0, 0.0, 0.0),
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
        material.clone(),
    )));
    //list.push(Box::new(Sphere::new(
    //    DVec3::new(-35.0, 4.0, -2000.0),
    //    400.0,
    //    material_sun.clone(),
    //)));
    let mut some_sphere = Box::new(Sphere::new(
        DVec3::new(-20.0, 10.0, -100.0),
        4.0,
        material_sun.clone(),
    ));
    some_sphere.translate(DVec3 {
        x: (-2.0),
        y: (2.0),
        z: (0.0),
    });
    some_sphere.scale(1.0);
    list.push(some_sphere);
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

struct Render {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
}

impl Render {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    ///

    fn draw(&self, frame: &mut [u8], count: f64) {
        //let a = frame;
        let channels = 4;
        let ray_per_pixel = 1;
        let camera = get_camera();
        let world_scene = random_scene();
        let mut result: Vec<Vec<u8>> = Vec::new();
        frame
            .par_chunks_mut((WIDTH as u32 * channels) as usize)
            .rev()
            .enumerate()
            .for_each(|(j, row)| {
                for (i, rgb) in row.chunks_mut(channels as usize).enumerate() {
                    let mut pixel_colour = DVec3::new(0.0, 0.0, 0.0);
                    let existing_r = rgb[0].clone();
                    let existing_g = rgb[1].clone();
                    let existing_b = rgb[2].clone();
                    let existing_colour =
                        DVec3::new(existing_r as f64, existing_g as f64, existing_b as f64);
                    for _ in 0..ray_per_pixel {
                        //println!("{:?}", random);
                        //process::exit(0);
                        let mut rng = rand::thread_rng();
                        let random_num = rng.gen::<f64>();
                        //println!("{:?}", random_x);
                        let u = (i as f64 + random_num) / WIDTH as f64;
                        let v = (j as f64 + random_num) / HEIGHT as f64;
                        let r = &camera.get_ray(u, v);
                        pixel_colour = (pixel_colour + ray_color(&r, &world_scene, 0)) * 255.0;
                        //pixel_colour = pixel_colour.lerp(existing_colour, (1.0 / count));
                        pixel_colour = existing_colour.lerp(pixel_colour, (1.0 / count));
                        rgb[0] = pixel_colour.x as u8;
                        rgb[1] = pixel_colour.y as u8;
                        rgb[2] = pixel_colour.z as u8;
                        rgb[3] = (255) as u8
                    }
                    //pixel_colour = pixel_colour / ray_per_pixel as f64;
                    //
                    //let red_result = (existing_r as f64 + (pixel_colour.x as f64 * 255.0)) as u8;
                    //let green_result = (existing_g as f64 + (pixel_colour.y as f64 * 255.0)) as u8;
                    //let blue_result = (existing_b as f64 + (pixel_colour.z as f64 * 255.0)) as u8;
                    //
                    //rgb[0] = red_result.clone();
                    //rgb[1] = green_result.clone();
                    //rgb[2] = blue_result.clone();
                    //rgb[3] = (255) as u8;
                }
            });
    }
}

fn get_camera() -> Camera {
    // Camera
    let lookfrom = DVec3::new(0.0, 5.0, 0.0);
    let lookat = DVec3::new(0.0, 0.0, -100.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 3.5_f64;
    let camera = Camera::new(
        lookfrom,
        lookat,
        DVec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(WIDTH) / f64::from(HEIGHT),
        aperture,
        dist_to_focus,
    );
    camera
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels + egui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        //let mut pixels = PixelsBuilder::new(WIDTH, HEIGHT, surface_texture);
        //pixels.texture_format(egui_wgpu::wgpu::TextureFormat::Rgba32Float);
        //let a = pixels.build()?;
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    //channels
    let channels = 3;
    //rendersettings
    let ray_per_pixel = 10;
    //scene
    let world_scene = random_scene();
    //Image

    let pb = ProgressBar::new(WIDTH as u64 * HEIGHT as u64 * ray_per_pixel.clone());

    // Camera
    let lookfrom = DVec3::new(0.0, 5.0, 0.0);
    let lookat = DVec3::new(0.0, 0.0, -100.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 3.5_f64;
    let camera = Camera::new(
        lookfrom,
        lookat,
        DVec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(WIDTH) / f64::from(HEIGHT),
        aperture,
        dist_to_focus,
    );

    //
    let mut render = Render::new();
    let mut count: f64 = 0.0;

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Update the scale factorxx
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            //\render.update();
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                println!("Window event");
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                //println!("Redraw Requested");
                // Draw the world
                //println!("Event::RedrawRequested");
                count += 1.0;
                if count < 100.0 {
                    render.draw(pixels.frame_mut(), count);
                }

                //image::save_buffer(
                //    format!(
                //        "D:/rust/raytracing_in_a_wekeend_rust/src/renders/test_{:?}.png",
                //        count
                //    ),
                //    buffer_pixels,
                //    WIDTH,
                //    HEIGHT,
                //    image::ColorType::Rgba8,
                //)
                //.expect("Failed to save output image");

                // Prepare egui
                framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {
                //println!("other");
                ()
            }
        }
    });
}
