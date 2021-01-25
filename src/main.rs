mod camera;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::io::{stderr, Write};

use rand;

use camera::Camera;
use hittable::Hittable;
use hittable_list::hittable_list_hit;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Color};

fn ray_color<T: Hittable>(r: Ray, world: &Vec<T>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = hittable_list_hit(&world, r, 0.001, f64::INFINITY) {
        // let target: Point3 = rec.p + vec3::random_in_hemisphere(rec.normal);
        // return ray_color(
        //     &Ray { orig: rec.p, dir: target - rec.p },
        //     world,
        //     depth-1)
        // * 0.5;
        match rec.material.scatter(r, rec) {
            Some((attenuation, scattered)) => {
                return attenuation * ray_color(scattered, world, depth-1);
            },
            None => {
                return Color::new(0, 0, 0);
            }
        };
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return Color::new(1, 1, 1) * (1.0 - t)
           + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // IMAGE
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // WORLD
    let mut world: Vec<Sphere> = vec!();

    let material_ground = Material::Lambertian { albedo: Color::new(0.8, 0.8, 0.0) };
    let material_center = Material::Lambertian { albedo: Color::new(0.7, 0.3, 0.3) };
    let material_left = Material::Metal { albedo: Color::new(0.8, 0.8, 0.8) };
    let material_right = Material::Metal { albedo: Color::new(0.8, 0.6, 0.2) };

    world.push(Sphere{
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
        material: material_ground,
    });
    world.push(Sphere{
        center: Point3::new(0., 0., -1.),
        radius: 0.5,
        material: material_center,
    });
    world.push(Sphere{
        center: Point3::new(-1., 0., -1.),
        radius: 0.5,
        material: material_left,
    });
    world.push(Sphere{
        center: Point3::new(1., 0., -1.),
        radius: 0.5,
        material: material_right,
    });

    // CAMERA
    let cam = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rlines remaining : {:>3}", j);
        let _ = stderr().flush();
        for i in 0..image_width {
            let mut pixel_color = Color::zero();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            println!("{}", vec3::write_color(&pixel_color, samples_per_pixel));
        }
    }
    eprintln!("\ndone");
}
