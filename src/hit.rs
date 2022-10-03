use std::rc::Rc;

use nalgebra::Vector3;
use super::ray::Ray;
// use crate::material::Material;
use super::material::Scatter;

pub struct HitRecord {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub front_face: bool,
    // pub material: &'a dyn Material
    pub material: Rc<dyn Scatter>
}

impl HitRecord{
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3<f32>){
        self.front_face = r.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face{
            outward_normal
        } else {
            -1.0 * outward_normal
        }
    }
}

pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;
// pub type World = Vec<Rc<dyn Hit>>;

impl Hit for World {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmp_rec = None;
        let mut closest_so_far = t_max;

        for object in self{
            if let Some(rec) = object.hit(r, t_min, closest_so_far){
                closest_so_far = rec.t;
                tmp_rec = Some(rec);
            }
        }
        tmp_rec
    }
}