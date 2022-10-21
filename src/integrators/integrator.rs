extern crate nalgebra_glm as glm;

use crate::ray::Ray;
use crate::scene::Scene;
use crate::surfaces::surface::Surface;
use enum_dispatch::enum_dispatch;
use glm::Vec3;

use serde_json::Value;

#[enum_dispatch]
pub trait Integrator {
    /// Sample the incident radiance along a ray
    fn li(&self, scene: &Scene, ray: &Ray, depth: i32) -> Vec3;

    /// To retrofit the code
    fn is_integrator(&self) -> bool { true }
}

#[enum_dispatch(Integrator)]
#[derive(Debug, Clone)]
pub enum IntegratorType {
    NotAnIntegrator,
    NormalsIntegrator,
}

#[derive(Debug, Clone)]
pub struct NotAnIntegrator;

impl Integrator for NotAnIntegrator {
    fn li(&self, _scene: &Scene, _ray: &Ray, _depth: i32) -> Vec3 {
            Vec3::zeros()
    }
    fn is_integrator(&self) -> bool { false }
}


#[derive(Debug, Clone)]
pub struct NormalsIntegrator;

impl Integrator for NormalsIntegrator {
    fn li(&self, scene: &Scene, ray: &Ray, _depth: i32) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            glm::abs(&hit.sn)
        } else {
            Vec3::zeros()
        }
    }
}

pub fn create_integrator(v: &Value) -> IntegratorType {
    let m = v.as_object().unwrap();
    if ! m.contains_key("integrator"){
        return IntegratorType::from(NotAnIntegrator {});
    } 
    let integrator_json = v.get("integrator").unwrap();
    let sampler_type = integrator_json
        .get("type")
        .expect("no integrator type")
        .as_str()
        .expect("lolz");

    match sampler_type {
        "normals" => {
            IntegratorType::from(NormalsIntegrator {})
        }
        _ => {
            unimplemented!("Sampler type {}", sampler_type);
        }
    }
}