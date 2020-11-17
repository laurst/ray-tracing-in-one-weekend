use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord{
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = if r.dir.dot(outward_normal) < 0. { true } else { false };
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
