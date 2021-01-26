use std::rc::Rc;

use crate::hittable;
use crate::material;
use crate::ray;
use crate::vec3;

use hittable::{Hittable, HitRecord};
use material::Material;
use ray::Ray;
use vec3::Color;

pub fn hittable_list_hit<T: Hittable>(objects: &Vec<T>, r: Ray, t_min: f64,
                     t_max: f64) -> Option<HitRecord> {
    let mut temp_rec = HitRecord::new(Rc::new(
        Material::Lambertian { albedo: Color::zero() }
    ));
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in objects {
        if let Some(rec) = object.hit(r, t_min, closest_so_far) {
            hit_anything = true;
            closest_so_far = rec.t;
            temp_rec = rec;
        }
    }

    if hit_anything { Some(temp_rec) } else { None }
}
