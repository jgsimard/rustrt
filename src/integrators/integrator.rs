extern crate nalgebra_glm as glm;

use crate::ray::Ray;
use crate::samplers::sampler::SamplerType;
use crate::scene::Scene;
use crate::utils::read_or;
use enum_dispatch::enum_dispatch;
use glm::Vec3;

use serde_json::Value;

use crate::integrators::ambiant_occlusion::AmbientOcclusionIntegrator;
use crate::integrators::normals::NormalsIntegrator;
use crate::integrators::path_tracer_mats::PathTracerMatsIntegrator;
use crate::integrators::path_tracer_mis::PathTracerMISIntegrator;
use crate::integrators::path_tracer_nee::PathTracerNEEIntegrator;

#[enum_dispatch]
pub trait Integrator {
    /// Sample the incident radiance along a ray
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray: &Ray, depth: i32) -> Vec3;
}

#[enum_dispatch(Integrator)]
#[derive(Debug, Clone)]
pub enum IntegratorType {
    Normals(NormalsIntegrator),
    AmbientOcclusion(AmbientOcclusionIntegrator),
    PathTracerMats(PathTracerMatsIntegrator),
    PathTracerNEE(PathTracerNEEIntegrator),
    PathTracerMIS(PathTracerMISIntegrator),
}

pub fn create_integrator(v: &Value) -> IntegratorType {
    let m = v.as_object().unwrap();
    if !m.contains_key("integrator") {
        return IntegratorType::from(PathTracerMatsIntegrator { max_bounces: 64 });
    }
    let integrator_json = v.get("integrator").unwrap();
    let sampler_type = integrator_json
        .get("type")
        .expect("no integrator type")
        .as_str()
        .expect("could not get integrator type");

    match sampler_type {
        "normals" => IntegratorType::from(NormalsIntegrator {}),
        "ao" => IntegratorType::from(AmbientOcclusionIntegrator {}),
        "path_tracer_mats" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 1);
            IntegratorType::from(PathTracerMatsIntegrator { max_bounces })
        }
        "path_tracer_nee" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 1);
            IntegratorType::from(PathTracerNEEIntegrator { max_bounces })
        }
        "path_tracer_mis" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 1);
            IntegratorType::from(PathTracerMISIntegrator { max_bounces })
        }
        _ => {
            unimplemented!("Sampler type {}", sampler_type);
        }
    }
}
