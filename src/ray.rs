extern crate nalgebra_glm as glm;
use glm::Vec3;

// use nalgebra::Unit;

// TODO : Use Unit for direction

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub mint: f32,
    pub maxt: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            mint: 0.0001, // TODO : maybe change this
            maxt: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
