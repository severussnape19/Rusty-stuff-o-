#![allow(unused)]
#![allow(dead_code)]

mod misc;
mod interval;

use std::{env, fs::{File, OpenOptions}, io::{BufWriter, Write}, ops::{Add, AddAssign, Div, Mul, Sub}, path::Path};
use glam::{Vec3, camera};
use crate::misc::{Camera, Color, HittableList, Point3, Ray, Sphere, write_color};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[BISHH] File path where bru?");
        std::process::exit(1);
    }
    // WORLD
    let mut world: HittableList = HittableList::default();
    world.add(Sphere::from(Point3::new(0., 0., -1.0), 0.5));
    world.add(Sphere::from(Point3::new(0., -100.5, -1.0), 100.));

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 1280;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length: f32 = 1.;
    let viewport_height: f32 = 2.0;

    let samples_per_pixel = 20;

    let mut camera: Camera = Camera::from(
        image_width,
        aspect_ratio,
        focal_length,
        viewport_height,
        samples_per_pixel
    );

    let path = Path::new(&args[1]);
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    let mut file = BufWriter::new(file);

    camera.render(&world, samples_per_pixel, &mut file);

    println!("Info:\nImage width: {} | Image height: {}", image_width, image_height);
    println!("Focal length: {}  | aspect ratio: {}", focal_length, aspect_ratio);
    println!("Top left corner: {:?}", camera.upper_left_corner);
    println!("Horizontal: {:?} | vertical: {:?}", camera.horizontal, camera.vertical);
    println!("Pixel delta_u: {:?} | delta_v: {:?}", camera.pixel_delta_u, camera.pixel_delta_v);

    Ok(())
}
