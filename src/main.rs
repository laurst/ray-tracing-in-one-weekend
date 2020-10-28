mod vec3;
mod ray;

use std::io::{stderr, Write};
use vec3::{Point3, Vec3, Color};

fn ray_color(ray: &ray::Ray) -> Color {
    let camera = Point3::new(0., 0., -1.);
    if hit_sphere(&camera, 0.5, ray) {
        return Color::new(1., 0., 0.);
    }

    let unit_direction = ray.dir.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);

    return Color::new(1.0, 1.0, 1.0).mul(1.0 - t)
           + Color::new(0.5, 0.7, 1.0).mul(t);
}

fn hit_sphere(center: &Point3, radius: f64, r: &ray::Ray) -> bool {
    let oc = r.orig - *center;
    let a = r.dir.dot(&r.dir);
    let b = 2.0 * oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner = origin
                            - horizontal.div(2.)
                            - vertical.div(2.)
                            - Vec3::new(0., 0., focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rlines remaining : {:>3}", j);
        stderr().flush();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = ray::Ray{
                orig: origin,
                dir: lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            };
            let pixel_color = ray_color(&r);
            vec3::write_color(&pixel_color);
        }
    }
    eprintln!("\ndone");
}
