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
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.;
        let h = (theta/2.).tan();
        let viewport_height: f64 = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f64 = 1.;
        let origin = Point3::zero();
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let lower_left_corner = origin
            - horizontal / 2.
            - vertical / 2.
            - Vec3::new(0., 0., focal_length);

        Camera{
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

    pub fn get_ray(&self, u: f64, v: f64) -> Ray{
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin,
        )
    }
}
