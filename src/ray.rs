use crate::vec3;

pub struct Ray {
    pub orig: vec3::Point3,
    pub dir: vec3::Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> vec3::Point3 {
        self.orig + self.dir.constant_mul(t)
    }
}
