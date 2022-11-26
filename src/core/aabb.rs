use crate::core::ray::Ray;
use nalgebra_glm::{any, equal, max2, min2, Vec3};
use std::mem;

/// A 3D axis-aligned bounding box consisting of two 3D points min and max
#[derive(Debug, PartialEq, Clone)]
pub struct Aabb {
    /// The lower-bound of the interval
    pub min: Vec3,
    /// The upper-bound of the interval
    pub max: Vec3,
}

impl Aabb {
    pub fn new() -> Aabb {
        Aabb {
            max: Vec3::new(f32::MIN, f32::MIN, f32::MIN),
            min: Vec3::new(f32::MAX, f32::MAX, f32::MAX),
        }
    }

    fn is_finite(&self) -> bool {
        const INF: Vec3 = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        const NEG_INF: Vec3 = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        !any(&equal(&self.min, &INF))
            || any(&equal(&self.min, &NEG_INF))
            || any(&equal(&self.max, &INF))
            || any(&equal(&self.max, &NEG_INF))
    }

    pub fn is_empty(&self) -> bool {
        (self.min.x > self.max.x) | (self.min.y > self.max.y) | (self.min.z > self.max.z)
    }

    pub fn enclose(&mut self, other: &Aabb) {
        self.min = min2(&self.min, &other.min);
        self.max = max2(&self.max, &other.max);
    }

    pub fn enclose_point(&mut self, point: &Vec3) {
        self.min = min2(&self.min, point);
        self.max = max2(&self.max, point);
    }

    pub fn center(&self) -> Vec3 {
        if !self.is_finite() {
            println!("min : {} max : {}", self.min, self.max);
            panic!("is not finite");
        }
        (self.min + self.max) / 2.0
    }

    pub fn diagonal(&self) -> Vec3 {
        self.max - self.min
    }

    // pub fn volume(&self) -> f32 {
    //     let d = self.diagonal();
    //     d.x * d.y * d.z
    // }

    // pub fn area(&self) -> f32 {
    //     let d = self.diagonal();
    //     let mut result = 0.0;
    //     for i in 0..3 {
    //         let mut term = 1.0;
    //         for j in 0..3 {
    //             if i == j {
    //                 continue;
    //             }
    //             term *= d[j];
    //         }
    //         result += term;
    //     }
    //     result * 2.0
    // }

    pub fn intersect(&self, ray: &Ray) -> bool {
        let mut min_t = ray.min_t;
        let mut max_t = ray.max_t;
        for i in 0..3 {
            let inv_d = 1.0 / ray.direction[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            min_t = if t0 > min_t { t0 } else { min_t };
            max_t = if t1 < max_t { t1 } else { max_t };
            if max_t < min_t {
                return false;
            }
        }
        true
    }
}
