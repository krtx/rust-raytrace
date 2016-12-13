use std::fmt;
use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl <'a>Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(mut self, other: &'a Vec3) -> Vec3 {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(mut self) -> Vec3 {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }
}

impl <'a>Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(mut self, other: &'a Vec3) -> Vec3 {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, other: f64) -> Vec3 {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, mut other: Vec3) -> Vec3 {
        other.x *= self;
        other.y *= self;
        other.z *= self;
        other
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        self * (1.0 / other)
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        (1.0 / self) * other
    }
}

impl Vec3 {
    pub fn size(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let size = self.size();
        self / size
    }
}

pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f64 {
    lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub unit_dir: Vec3,
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.origin, self.unit_dir)
    }
}
