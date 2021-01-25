use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{ Color, random_unit_vector, random_in_unit_sphere, refract };

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal {
        albedo: Color,
        fuzz: f64,
    },
    Dielectric { index_of_refraction: f64 },
}


impl Material {
    pub fn scatter(self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + random_unit_vector();
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction);
                return Some((albedo, scattered));
            },
            Material::Metal { albedo, fuzz } => {
                let reflected = r_in.dir.unit_vector().reflect(rec.normal);
                let fuzz = if fuzz < 1. { 1. } else { fuzz };
                let scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * fuzz);
                let attenuation = albedo;
                return if scattered.dir.dot(rec.normal) > 0. {
                    Some((attenuation, scattered))
                } else {
                    None
                };
            },
            Material::Dielectric { index_of_refraction: ir } => {
                let attenuation = Color::new(1, 1, 1);
                let refraction_ratio = if rec.front_face { 1. / ir } else { ir };

                let unit_direction = r_in.dir.unit_vector();
                let refracted = refract(unit_direction, rec.normal, refraction_ratio);

                let scattered = Ray::new(rec.p, refracted);
                return Some((attenuation, scattered));
            },
        }
    }
}
