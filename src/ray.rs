use crate::vec3::{Point, Vec3};

pub struct Ray {
    orig: Point,
    dir: Vec3,
}
impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }
    pub fn origin(&self) -> Point {
        self.orig
    }
    pub fn direction(&self) -> Point {
        self.dir
    }
    pub fn at(&self, t: f32) -> Point {
        self.orig + (self.dir * t)
    }
}
