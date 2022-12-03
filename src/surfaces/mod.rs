mod bvh;
mod quad;
mod sphere;
mod surface_group;
mod triangle;

use enum_dispatch::enum_dispatch;
use nalgebra_glm::{Vec2, Vec3};
use serde_json::{Map, Value};
use std::sync::Arc;

use crate::core::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::utils::{read_or, Factory};
use crate::materials::{MaterialFactory, MaterialType};

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
    pub mat: Arc<MaterialType>,
}

/// Data record for conveniently querying and sampling emitters
pub struct EmitterRecord {
    /// Origin point from which we sample the emitter
    pub o: Vec3,
    /// Direction vector from 'o' to 'hit.p
    pub wi: Vec3,
    /// Solid angle density wrt. 'o'
    pub pdf: f32,
    /// Hit information at the sampled point
    pub hit: HitInfo,
    /// Emitted Value
    pub emitted: Vec3,
}

/// Data record for querying the scatter ray `is_specular` is for backward compatibility
pub struct ScatterRecord {
    /// Attenuation to apply to the traced ray
    pub attenuation: Vec3,
    /// The sampled outgoing direction
    pub wo: Vec3,
    /// Flag indicating whether the ray has a degenerate PDF  
    pub is_specular: bool,
}

/// This is the trait for all surfaces.
///
/// Surfaces represent the geometry of the scene. A Surface could be an individual primitive like a #Sphere, or it could
/// be composed of many smaller primitives, e.g. the triangles composing a #Mesh.
#[enum_dispatch]
pub trait Surface {
    /// Ray-Surface intersection test.
    ///
    /// Intersect a ray against this surface and return detailed intersection information.
    fn intersect(&self, ray: &Ray) -> Option<HitInfo>;

    /// Return the surface's world-space AABB.
    fn bounds(&self) -> Aabb {
        unimplemented!();
    }

    /// Sample a direction from `rec.o` towards this surface.
    ///
    /// Store result in `rec`, and return important weight (i.e. the color of the Surface divided by the probability
    /// density of the sample with respect to solid angle).
    fn sample(&self, o: &Vec3, rv: Vec2) -> Option<EmitterRecord>;

    /// TODO
    fn sample_from_group(&self, _o: &Vec3, _rv: Vec2, _rv1: f32) -> Option<EmitterRecord> {
        unimplemented!()
    }

    /// TODO
    fn pdf_child(&self, _o: &Vec3, _dir: &Vec3, _rv: f32) -> f32 {
        unimplemented!()
    }

    /// Return the probability density of the sample generated by #sample
    fn pdf(&self, o: &Vec3, dir: &Vec3) -> f32;

    /// Return whether or not this Surface's Material is emissive.
    fn is_emissive(&self) -> bool;
}

use crate::surfaces::bvh::{Bvh, SplitMethod};
use crate::surfaces::quad::Quad;
use crate::surfaces::sphere::Sphere;
use crate::surfaces::triangle::{Mesh, Triangle};

#[enum_dispatch(Surface)]
#[derive(Debug, PartialEq, Clone)]
pub enum SurfaceType {
    Sphere,
    Quad,
    Triangle,
    Bvh,
}

pub struct SurfaceFactory {
    pub material_factory: MaterialFactory,
}

impl Factory<SurfaceType> for SurfaceFactory {
    fn make(&mut self, v: &Value) -> Option<Vec<SurfaceType>> {
        let m = v.as_object().unwrap();
        let surface_type = m.get("type").unwrap().as_str().unwrap();

        let vec_surfaces = match surface_type {
            "sphere" => vec![SurfaceType::from(Sphere::new(v, self))],
            "quad" => vec![SurfaceType::from(Quad::new(v, self))],
            "triangle" => vec![SurfaceType::from(Triangle::new(v, self))],
            "mesh" => Mesh::read(v, self),
            "group" => {
                let Some(children) = m.get("children") else {
                    panic!("No children to render :(");
                };
                children
                    .as_array()
                    .expect("children should be in an array")
                    .iter()
                    .flat_map(|sur| {
                        self.make(sur).unwrap_or_else(|| {
                            panic!("surface of type : {} not yet supported", sur["type"])
                        })
                    })
                    .collect()
            }
            _ => unimplemented!("surface type {} not supported", surface_type),
        };
        Some(vec_surfaces)
    }
}

impl SurfaceFactory {
    pub fn get_material(&self, m: &Map<String, Value>) -> Arc<MaterialType> {
        let material = if let Some(mat) = m.get("material") {
            if mat.is_string() {
                (*self
                    .material_factory
                    .materials
                    .get(mat.as_str().unwrap())
                    .unwrap())
                .clone()
            } else {
                self.material_factory.create_material(mat)
            }
        } else {
            panic!("Invalid material");
        };
        material
    }
}

use crate::surfaces::surface_group::LinearSurfaceGroup;

#[enum_dispatch(Surface)]
#[derive(Debug, PartialEq, Clone)]
pub enum SurfaceGroupType {
    LinearSurfaceGroup,
    Bvh,
}

pub fn create_surface_group(
    map: &Map<String, Value>,
    surfaces: &mut Vec<SurfaceType>,
) -> SurfaceGroupType {
    if let Some(accel_value) = map.get("accelerator") {
        let type_acceletator = accel_value.get("type").unwrap().as_str().unwrap();
        match type_acceletator {
            "bbh" => {
                let split_method = read_or(accel_value, "split_method", SplitMethod::Middle);
                SurfaceGroupType::from(Bvh::new(surfaces, &split_method))
            }
            _ => panic!("Unusported accelerator {}", type_acceletator),
        }
    } else {
        // default to a naive linear accelerator
        SurfaceGroupType::from(LinearSurfaceGroup {
            surfaces: surfaces.clone(),
        })
    }
}
