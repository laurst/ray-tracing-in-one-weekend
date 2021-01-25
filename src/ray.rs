use crate::vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn new(orig: vec3::Point3, dir: vec3::Vec3) -> Self {
        Ray { orig: orig, dir: dir }
    }
    pub fn at(self, t: f64) -> vec3::Point3 {
        self.orig + self.dir * t
    }
}
