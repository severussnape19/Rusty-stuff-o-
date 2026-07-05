use glam::Vec3;

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub origin: Point3,
    pub upper_left_corner: Point3,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub vertical: Vec3, // viewport_v
    pub horizontal: Vec3, // viewport_u
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Point3,
}

impl  Camera {
    pub fn from(
        image_width: i32, aspect_ratio: f32,
        focal_length: f32, viewport_height: f32
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

        Self {
            aspect_ratio,
            origin: camera_origin,
            upper_left_corner,
            viewport_height,
            viewport_width,
            vertical: vertical_v,
            horizontal: horizontal_u,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc
        }
    }
}

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
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

    pub fn ray_color(&self, sphere: &Sphere) -> Color {
        let t: f32 = Sphere::hit_sphere(sphere, self);
        if (t > 0.0f32) {
            let normal: Vec3 = (self.at(t) - Vec3::new(0., 0., -1.)).normalize();
            return 0.5 * (Color::from(normal) + 1.);
        }

        let unit_direction: Vec3 = Vec3::normalize(self.direction);
        let v: f32 = 0.5 * (unit_direction.y) + 1.0;

        let color_a = Color::new(1.0, 1., 1.);
        let color_b = Color::new(0.4, 0.7, 1.0);

        // lerp
        ((1.0 - v) * color_a) + (v * color_b)
    }
}

#[derive(Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
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
        let a: f32 = ray.direction.dot(ray.direction);
        let b: f32 = -2.0f32 * oc.dot(ray.direction);
        let c: f32 = oc.dot(oc) - self.radius * self.radius;

        let delta: f32 = b * b - (4.0f32 * a * c);

        if delta < 0.0f32 {
            -1.0f32
        } else {
            (-b - delta.sqrt() / (2.0f32 * a)) }
    }
}
