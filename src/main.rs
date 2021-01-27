mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::io::{stderr, Write};
use std::rc::Rc;

use rand::{random, thread_rng, Rng};

use camera::Camera;
use hittable::Hittable;
use hittable_list::hittable_list_hit;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

fn ray_color<T: Hittable>(r: Ray, world: &Vec<T>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = hittable_list_hit(&world, r, 0.001, f64::INFINITY) {
        match rec.material.scatter(r, rec) {
            Some((attenuation, scattered)) => {
                return attenuation * ray_color(scattered, world, depth - 1);
            }
            None => {
                return Color::new(0, 0, 0);
            }
        };
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return Color::new(1, 1, 1) * (1.0 - t) + Color::new(0.5, 0.7, 1) * t;
}

fn random_scene() -> Vec<Sphere> {
    let mut world = vec![];

    let material_ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    world.push(Sphere {
        center: Point3::new(0, -1000, 0),
        radius: 1000.,
        material: Rc::new(material_ground),
    });

    let radius = 0.2;
    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian { albedo };
                    world.push(Sphere {
                        center,
                        radius,
                        material: Rc::new(sphere_material)
                    });
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = rng.gen_range(0., 0.5);
                    let sphere_material = Material::Metal { albedo, fuzz };
                    world.push(Sphere {
                        center,
                        radius,
                        material: Rc::new(sphere_material)
                    });
                } else {
                    let sphere_material = Material::Dielectric {
                        index_of_refraction: 1.5
                    };
                    world.push(Sphere {
                        center,
                        radius,
                        material: Rc::new(sphere_material)
                    });
                }
            }
        }
    }

    let material1 = Material::Dielectric {
        index_of_refraction: 1.5,
    };
    let material2 = Material::Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    };
    let material3 = Material::Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    world.push(Sphere {
        center: Point3::new(0, 1, 0),
        radius: 1.,
        material: Rc::new(material1),
    });
    world.push(Sphere {
        center: Point3::new(-4, 1, 0),
        radius: 1.,
        material: Rc::new(material2),
    });
    world.push(Sphere {
        center: Point3::new(4, 1, 0),
        radius: 1.,
        material: Rc::new(material3),
    });

    world
}

fn main() {
    // IMAGE
    let aspect_ratio = 3. / 2.;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // WORLD
    let world = random_scene();

    // CAMERA
    let lookfrom = Point3::new(13, 2, 3);
    let lookat = Point3::new(0, 0, 0);
    let vup = Vec3::new(0, 1, 0);
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rlines remaining : {:>3}", j);
        let _ = stderr().flush();
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            println!("{}", vec3::write_color(&pixel_color, samples_per_pixel));
        }
    }
    eprintln!("\ndone");
}
