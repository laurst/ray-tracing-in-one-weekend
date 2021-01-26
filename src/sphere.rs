use std::rc::Rc;

use crate::hittable;
use crate::material;
use crate::ray;
use crate::vec3;

use hittable::{HitRecord, Hittable};
use material::Material;
use ray::Ray;
use vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a*c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::new(self.material.clone());
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return Some(rec);
    }
}
