use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3(self * vec.0, self * vec.1, self * vec.2)
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        let k = 1. / other;
        Vec3(self.0 * k, self.1 * k, self.2 * k)
    }
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit_vector(&self) -> Self {
        let k = 1. / self.length();
        Vec3(self.0 * k, self.1 * k, self.2 * k)
    }
}

pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - a.2 * b.1,
        -(a.0 * b.2 - a.2 * b.0),
        a.0 * b.1 - a.1 * b.0,
    )
}
