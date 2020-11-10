mod hittable;
mod ray;
mod vec3;

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(&r.dir);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a*c;

        if discriminant > 0. {
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root) / a);
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
