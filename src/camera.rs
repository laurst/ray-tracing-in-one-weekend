use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    vfov: f64,
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let h = (theta / 2.).tan();
        let viewport_height: f64 = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            vfov,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        )
    }
}
