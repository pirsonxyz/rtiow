use std::ops::{Add, AddAssign, DivAssign, Index, MulAssign, Sub};
pub struct Vec3 {
    pub e: [f32; 3],
}
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
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
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

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
