#![allow(unused)]
#![allow(dead_code)]

mod misc;
use std::{env, fs::{File, OpenOptions}, io::{BufWriter, Write}, ops::{Add, AddAssign, Div, Mul, Sub}, path::Path};
use glam::{Vec3, camera};

use crate::misc::{Camera, Color, Point3, Ray, Sphere};

fn write_color(file: &mut dyn Write, color: Color) {
    let r: f32 = color.x;
    let g: f32 = color.y;
    let b: f32 = color.z;

    let ir = ((r * 255.999) as i32).clamp(0, 255);
    let ig = ((g * 255.999) as i32).clamp(0, 255);
    let ib = ((b * 255.999) as i32).clamp(0, 255);
    writeln!(file, "{} {} {}", ir, ig, ib);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[BISHH] File path where bru?");
        std::process::exit(1);
    }

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 800;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;

    let camera: Camera = Camera::from(image_width, aspect_ratio, focal_length, viewport_height);
    let sphere: Sphere = Sphere::from(Point3::new(0., 0., -1.0), 0.5);

    let path = Path::new(&args[1]);

    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;

    let mut file = BufWriter::new(file);
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    for j in 0..image_height {

        let percentage = (j as f32 / (image_height - 1) as f32);
        println!("Percentage done: {}", percentage * 100.0);

        for i in 0..image_width {
            let pixel_center = camera.pixel00_loc
                + (i as f32 * camera.pixel_delta_u)
                + (j as f32 * camera.pixel_delta_v);
            let ray_direction = pixel_center - camera.origin;

            let ray = Ray::from(camera.origin, ray_direction);

            let pixel_color = ray.ray_color(&sphere);
            write_color(&mut file, pixel_color);
        }
    }

    println!("Info:\nImage width: {} | Image height: {}", image_width, image_height);
    println!("Focal length: {}  | aspect ratio: {}", focal_length, aspect_ratio);
    println!("Top left corner: {:?}", camera.upper_left_corner);
    println!("Horizontal: {:?} | vertical: {:?}", camera.horizontal, camera.vertical);
    println!("Pixel delta_u: {:?} | delta_v: {:?}", camera.pixel_delta_u, camera.pixel_delta_v);

    Ok(())
}
