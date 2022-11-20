extern crate nalgebra_glm as glm;
use glm::Vec3;
use std::sync::atomic::Ordering;

use crate::core::utils::RAYS;

// use nalgebra::Unit;

// TODO : Use Unit for direction

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub min_t: f32,
    pub max_t: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        RAYS.fetch_add(1, Ordering::SeqCst);
        Ray {
            origin,
            direction,
            min_t: 0.0001, // TODO : maybe change this
            max_t: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
