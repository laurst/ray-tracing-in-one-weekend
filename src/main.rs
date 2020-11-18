mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use std::io::{stderr, Write};

use rand;

use camera::Camera;
use hittable::Hittable;
use hittable_list::hittable_list_hit;
use ray::Ray;
use sphere::Sphere;
use vec3::{Point3, Color};

fn ray_color<T: Hittable>(r: &Ray, world: &Vec<T>) -> Color {
    if let Some(rec) = hittable_list_hit(world, r, 0., f64::INFINITY) {
        return (rec.normal + Color::new(1., 1., 1.)) * 0.5;
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return Color::new(1.0, 1.0, 1.0) * (1.0 - t)
           + Color::new(0.5, 0.7, 1.0) * t;
}

fn main() {
    // IMAGE
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // WORLD
    let mut world: Vec<Sphere> = vec!();
    world.push(Sphere{
        center:Point3::new(0., 0., -1.),
        radius: 0.5,
    });
    world.push(Sphere{
        center:Point3::new(0., -100.5, -1.),
        radius: 100.,
    });

    // CAMERA
    let cam = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rlines remaining : {:>3}", j);
        let _ = stderr().flush();
        for i in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }
            println!("{}", vec3::write_color(&pixel_color, samples_per_pixel));
        }
    }
    eprintln!("\ndone");
}
