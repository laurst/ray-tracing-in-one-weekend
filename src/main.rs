mod vec3;
mod ray;

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;

fn main() {
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for j in (0..HEIGHT).rev() {
        eprint!("\rlines remaining : {}", j);
        for i in 0..WIDTH {
            let color = vec3::Color{
                x: i as f64 / (WIDTH-1) as f64,
                y: j as f64 / (HEIGHT-1) as f64,
                z: 0.25,
            };
            vec3::write_color(&color);
        }
    }
    eprintln!("\ndone");
}
