use crate::materials::material::Material;
use crate::ray::Ray;
extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use std::rc::Rc;

use crate::aabb::Aabb;

/// This is the abstract superclass for all surfaces.
///
/// Surfaces represent the geometry of the scene. A Surface could be an individual primitive like a #Sphere, or it could
/// be composed of many smaller primitives, e.g. the triangles composing a #Mesh.
pub trait Surface {
    // fn build_surface();

    /// Ray-Surface intersection test.
    ///
    /// Intersect a ray against this surface and return detailed intersection information.
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;

    /// Return the surface's world-space AABB.
    fn bounds(&self) -> Aabb {
        unimplemented!();
    }

    // fn sample(emit_rec: &EmitterRecord, rv: &Vec2) -> Vec3;
    // fn pdf(emit_rec: &EmitterRecord, rv: &Vec2) -> f32;
    // fn is_emissive() -> bool;
}

/// Contains information about a ray intersection hit point.
///
/// Used by surface intersection routines to return more than just a single value. Includes the position, traveled ray
/// distance, uv coordinates, the geometric and interpolated shading normals, and a pointer to the intersected surface
/// and underlying material.
pub struct HitInfo {
    /// Ray parameter for the hit
    pub t: f32,
    /// Hit position            
    pub p: Vec3,
    /// Geometric normal   
    pub gn: Vec3,
    /// Interpolated shading normal
    pub sn: Vec3,
    /// UV texture coordinates
    pub uv: Vec2,
    /// Material at the hit point
    pub mat: Rc<dyn Material>,
}

// /// Data record for conveniently querying and sampling emitters
// pub struct EmitterRecord
// {
//     /// Origin point from which we sample the emitter
//     o: Vec3,
//     /// Direction vector from 'o' to 'hit.p
//     wi: Vec3,
//     /// Solid angle density wrt. 'o'
//     pdf: f32,
//     /// Hit information at the sampled point
//     hit: HitInfo
// }
