use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{ Color, random_unit_vector };

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}


impl Material {
    pub fn scatter(self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray {
                    orig: rec.p,
                    dir: scatter_direction,
                };
                return Some((albedo, scattered));
            },
            Material::Metal { albedo } => {
                let reflected = r_in.dir.unit_vector().reflect(rec.normal);
                let scattered = Ray {
                    orig: rec.p,
                    dir: reflected,
                };
                let attenuation = albedo;
                return if scattered.dir.dot(rec.normal) > 0. {
                    Some((attenuation, scattered))
                } else {
                    None
                };
            },
        }
    }
}
