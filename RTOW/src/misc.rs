use core::f32;
use std::{f32::consts::PI, io::Write};
use glam::Vec3;
use indicatif::ParallelProgressIterator;
use rand::{RngExt, rngs::ThreadRng};

use crate::interval::Interval;

use rayon::prelude::*;

// -------------- uhhh? ---------------- //

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn write_color(file: &mut dyn Write, color: Color) {
    let r: f32 = color.x;
    let g: f32 = color.y;
    let b: f32 = color.z;

    let ir = ((r * 255.999) as i32).clamp(0, 255);
    let ig = ((g * 255.999) as i32).clamp(0, 255);
    let ib = ((b * 255.999) as i32).clamp(0, 255);
    writeln!(file, "{} {} {}", ir, ig, ib);
}

pub fn degrees_to_radians(degrees: f32) -> f32 { degrees * std::f32::consts::PI / 180. }

pub fn random_f32(rng: &mut ThreadRng) -> f32 {
    rng.random_range(0.0..=1.)
}

pub fn random_f32_range(rng: &mut ThreadRng, interval: &Interval) -> f32 {
    rng.random_range(interval.min..interval.max)
}

pub fn random_f32_vec3(rng: &mut ThreadRng) -> Vec3 {
    Vec3::new(
        rng.random_range(0.0..=1.),
        rng.random_range(0.0..=1.),
        rng.random_range(0.0..=1.)
    )
}

pub fn random_f32_vec3_range(rng: &mut ThreadRng, interval: &Interval) -> Vec3 {
    Vec3::new(
        rng.random_range(interval.min..interval.max),
        rng.random_range(interval.min..interval.max),
        rng.random_range(interval.min..interval.max),
    )
}

pub fn random_unit_vector() -> Vec3 {
    let mut rng = rand::rng();
    let interval: Interval = Interval::from(-1., 1.);
    loop {
        let p = random_f32_vec3_range(&mut rng, &interval);
        let length_squared = p.length_squared();
        if length_squared <= 1.0 && 1e-160 < length_squared {
            return p / length_squared.sqrt();
        }
    }
}

pub fn random_on_hemisphere(surface_normal: &Vec3) -> Vec3 {
    let on_unit_sphere: Vec3 = random_unit_vector();
    if (on_unit_sphere.dot(*surface_normal) > 0.0) {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}
// -------------- structs ---------------- //

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub origin: Point3,
    pub upper_left_corner: Point3,
    pub image_width: i32,
    pub image_height: i32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub vertical: Vec3, // viewport_v
    pub horizontal: Vec3, // viewport_u
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Point3,
    samples_per_pixel: i32,
    pixels_sample_scale: f32,
}

#[derive(Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

#[derive(Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

// -------------- Traits ---------------- //

pub trait Hittable {
fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
}

// -------------- Implementations ---------------- //

impl  Camera {
    pub fn from(
        image_width: i32, aspect_ratio: f32,
        focal_length: f32, viewport_height: f32,
        samples_per_pixel: i32,
    ) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let focal_length = Vec3::new(0., 0., focal_length);

        let viewport_aspect_ratio: f32 = image_width as f32 / image_height as f32;
        let viewport_width: f32 = aspect_ratio * viewport_height;

        let camera_origin = Point3::new(0., 0., 0.);

        let horizontal_u = Vec3::new(viewport_width, 0., 0.);
        let vertical_v   = Vec3::new(0., -viewport_height, 0.);

        let pixel_delta_u = horizontal_u / image_width as f32;
        let pixel_delta_v = vertical_v / image_height as f32;

        let upper_left_corner =
            camera_origin - focal_length - (horizontal_u / 2.) - (vertical_v / 2.);

        let pixel00_loc = upper_left_corner + 0.5 * (pixel_delta_v + pixel_delta_u);

        let pixels_sample_scale: f32 = 1.0 / samples_per_pixel as f32;

        Self {
            aspect_ratio,
            origin: camera_origin,
            upper_left_corner,
            image_width,
            image_height,
            viewport_height,
            viewport_width,
            vertical: vertical_v,
            horizontal: horizontal_u,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            pixels_sample_scale,
        }
    }

    pub fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        let mut hit_record: HitRecord = HitRecord::default();
        let interval: Interval = Interval::from(0., f32::INFINITY);
        if (world.hit(ray, &interval, &mut hit_record)) {
            let direction: Vec3 = random_on_hemisphere(&hit_record.normal);
            let ray = Ray::from(hit_record.p, direction);
            return 0.5 * self.ray_color(&ray, world);
        }

        let unit_direction: Vec3 = Vec3::normalize(ray.direction);
        let v: f32 = 0.5 * (unit_direction.y + 1.0);

        let color_a = Color::new(1.0, 1.0, 1.0);
        let color_b = Color::new(0.4, 0.7, 1.0);

        color_a.lerp(color_b, v)
    }

    fn sample_square(rng: &mut ThreadRng) -> Vec3 {
        Vec3::new(
            random_f32(rng) - 0.5,
            random_f32(rng) - 0.5,
            0.)
    }

    pub fn get_ray(&self, i: i32, j: i32, local_rng: &mut ThreadRng) -> Ray {

        let offset = Self::sample_square(local_rng);
        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.origin;
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(ray_origin, ray_direction)
    }

    pub fn render(
        &self,
        world: &HittableList,
        samples_per_pixel: i32,
        file: &mut dyn Write
    ) {
        assert!(self.image_height > 0);

        writeln!(file, "P3\n{} {}\n255", self.image_width, self.image_height);

        let image_data: Vec<Vec<Color>> = (0..self.image_height)
            .into_par_iter()
            .progress_count(self.image_height as u64)
            .map(|j| {
                let mut local_rng = rand::rng();
                let mut row_colors = Vec::with_capacity(self.image_width as usize);

                for i in 0..self.image_width {
                    let mut pixel_color: Color = Color::default();

                    for _sample in 0..samples_per_pixel {

                        let ray = self.get_ray(i, j, &mut local_rng);
                        pixel_color += self.ray_color(&ray, world);
                    }
                    row_colors.push(pixel_color * self.pixels_sample_scale);
                }
                row_colors
            }).collect();

        for row in image_data {
            for color in row {
                write_color(file, color);
            }
        }
    }
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::new(0., 0., 0.),
            direction: Vec3::new(0., 0., 0.)
        }
    }

    pub fn from(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin, direction
        }
    }

    pub fn origin(&self) -> &Point3 { &self.origin }
    pub fn direction(&self) -> &Vec3 { &self.direction }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }


}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(*outward_normal) < 0.;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) }
    }
}

impl Sphere {
    pub fn from(center: Point3, radius: f32) -> Self {
        Self {
            center,
            radius
        }
    }

    pub fn hit_sphere(&self, ray: &Ray) -> f32 {
        let oc: Point3 = self.center - ray.origin;
        let a: f32 = ray.direction.length_squared();
        let h: f32 = oc.dot(ray.direction);
        let c: f32 = oc.length_squared() - self.radius * self.radius;

        let delta: f32 = h * h - a * c;

        if delta < 0.0f32 {
            -1.0f32
        } else {
            (h - delta.sqrt()) / a
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
    let oc: Point3 = self.center - ray.origin;
        let a: f32 = ray.direction.length_squared();
        let h: f32 = oc.dot(ray.direction);
        let c: f32 = oc.length_squared() - self.radius * self.radius;
        let delta: f32 = h * h - a * c;

        if delta < 0. {
            return false;
        }

        let sqrt_delta: f32 = delta.sqrt();

        let mut root = (h - sqrt_delta) / a;
        if (root <= ray_t.min || root >= ray_t.max) {
            root = (h + sqrt_delta) / a;
            if (root <= ray_t.min || root >= ray_t.max) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        true
    }
}

impl HittableList {
    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static + Send + Sync
    {
        self.objects.push(Box::new(object));
    }

    pub fn clear(self) { /* consumes object */ }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::default();
        let mut hit_anything: bool = false;
        let mut closest: f32 = ray_t.max;

        for obj in self.objects.iter() {
            let interval: Interval = Interval::from(ray_t.min, closest);
            if obj.hit(ray, &interval, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.t;

                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
                rec.normal = temp_rec.normal;
                rec.p = temp_rec.p;
            }
        }

        hit_anything
    }
}
