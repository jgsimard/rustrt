use std::rc::Rc;

use nalgebra::Vector3;
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use super::material::Scatter;

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: Rc<dyn Scatter>
}

impl Sphere {
    pub fn new(center: Vector3<f32>, r: f32, m: Rc<dyn Scatter>) -> Sphere {
        Sphere {
            center: center,
            radius: r,
            material: m
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {

        let oc = r.origin - self.center;
    
        let a = r.direction.norm_squared();
        let half_b =  oc.dot(&r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        
        if discriminant < 0.0{
            return None;
        } 
        
        // Find the nearest root that lies in the acceptable range
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vector3::new(0.0, 0.0, 0.0),
            front_face: false,
            material: self.material.clone()
        };
    
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)       
    }
}
