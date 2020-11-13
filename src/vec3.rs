use std::ops;

pub type Color = Vec3;
pub type Point3 = Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3{x, y, z}
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        let len = self.length();
        Vec3{
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, t: f64) -> Self {
        Vec3{
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        Vec3{
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3{
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

pub fn write_color(v: &Vec3) {
    println!("{} {} {}",
             (255.999 * v.x) as u32,
             (255.999 * v.y) as u32,
             (255.999 * v.z) as u32
    );
}
