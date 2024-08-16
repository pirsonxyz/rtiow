use core::fmt;
use std::{
    fmt::write,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub},
};
#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub e: [f32; 3],
}
pub type Point = Vec3;
impl Vec3 {
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
    pub fn with_contents(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f32 {
        self[0]
    }
    pub fn y(&self) -> f32 {
        self[1]
    }
    pub fn z(&self) -> f32 {
        self[2]
    }
    pub fn length(&self) -> f32 {
        return f32::sqrt(self.lenght_squared());
    }
    pub fn lenght_squared(&self) -> f32 {
        self[0].powi(2) + self[1].powi(2) + self[2].powi(2)
    }
    pub fn unit_vector(v: Vec3) -> Self {
        v / v.length()
    }
    #[inline]
    pub fn double_dot(v: Vec3, u: Vec3) -> f32 {
        (v[0] * u[0]) + (v[1] * u[1]) + (v[2] + u[2])
    }
    #[inline]
    pub fn cross(v: Vec3, u: Vec3) -> Self {
        Self {
            e: [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}
impl Sub<f32> for Vec3 {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Self {
            e: [self[0] - other, self[1] - other, self[2] - other],
        }
    }
}
impl Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Self {
            e: [self[0] + other, self[1] + other, self[2] + other],
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        };
    }
}
impl AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        *self = Self {
            e: [self[0] + other, self[1] + other, self[2] + other],
        };
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        };
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            e: [self[0] * other, self[1] * other, self[2] * other],
        };
    }
}
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            e: [self[0] / other[0], self[1] / other[1], self[2] / other[2]],
        };
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self {
            e: [self[0] / other, self[1] / other, self[2] / other],
        };
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            e: [rhs * self[0], rhs * self[1], rhs * self[2]],
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::with_contents(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self * (rhs / 1.0)
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}
