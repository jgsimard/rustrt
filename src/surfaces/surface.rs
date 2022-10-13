use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::ray::Ray;
use nalgebra::{Vector2, Vector3};
use std::rc::Rc;

use crate::aabb::Aabb;

pub trait Surface {
    // fn build_surface();
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;
    fn bounds(&self) -> Aabb {unimplemented!();}
    // fn local_bounds(&self) -> Box3;
    // fn sample(emit_rec: &EmitterRecord, rv: &Vector2<f32>) -> Vector3<f32>;
    // fn pdf(emit_rec: &EmitterRecord, rv: &Vector2<f32>) -> f32;
    // fn is_emissive() -> bool;
}

pub struct HitInfo {
    /// Ray parameter for the hit
    pub t: f32,
    /// Hit position            
    pub p: Vector3<f32>,
    /// Geometric normal   
    pub gn: Vector3<f32>,
    /// Interpolated shading normal
    pub sn: Vector3<f32>,
    /// UV texture coordinates
    pub uv: Vector2<f32>,
    /// Material at the hit point
    pub mat: Rc<dyn Material>,
}

// TODO : CHANGE THIS< THIS IS HORRIBLE
impl HitInfo {
    pub fn empty() -> HitInfo {
        HitInfo {
            t: -1.,
            p: Default::default(),
            gn: Default::default(),
            sn: Default::default(),
            uv: Default::default(),
            mat: Rc::new(Lambertian {
                albedo: Vector3::x(),
            }),
        }
    }
}

// /// Data record for conveniently querying and sampling emitters
// pub struct EmitterRecord
// {
//     /// Origin point from which we sample the emitter
//     o: Vector3<f32>,
//     /// Direction vector from 'o' to 'hit.p
//     wi: Vector3<f32>,
//     /// Solid angle density wrt. 'o'
//     pdf: f32,
//     /// Hit information at the sampled point
//     hit: HitInfo
// }
