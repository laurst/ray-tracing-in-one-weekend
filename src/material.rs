use rand;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{ Color, random_unit_vector, random_in_unit_sphere, refract, reflect };

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
                let reflected = reflect(r_in.dir.unit_vector(), rec.normal);
                let fuzz = if fuzz > 1. { 1. } else { fuzz };
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
                let cos_theta = rec.normal.dot(-unit_direction).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
                {
                    reflect(unit_direction, rec.normal)
                } else {
                    refract(unit_direction, rec.normal, refraction_ratio)
                };

                let scattered = Ray::new(rec.p, direction);

                return Some((attenuation, scattered));
            },
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.-ref_idx) / (1.+ref_idx)).powi(2);
    r0 + (1.-r0) * (1.-cosine).powi(5)
}
