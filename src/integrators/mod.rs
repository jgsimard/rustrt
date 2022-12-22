mod ambiant_occlusion;
mod normals;
mod path_tracer_mats;
mod path_tracer_mis;
mod path_tracer_nee;

use enum_dispatch::enum_dispatch;
use nalgebra_glm::Vec3;
use rand::Rng;
use serde_json::{Map, Value};

use crate::core::ray::Ray;
use crate::core::scene::Scene;
use crate::core::utils::read_or;
use crate::samplers::SamplerType;

#[enum_dispatch]
pub trait Integrator {
    /// Sample the incident radiance along a ray
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, rng: &mut impl Rng, ray: &Ray) -> Vec3;
}

use crate::integrators::ambiant_occlusion::AmbientOcclusionIntegrator;
use crate::integrators::normals::NormalsIntegrator;
use crate::integrators::path_tracer_mats::PathTracerMatsIntegrator;
use crate::integrators::path_tracer_mis::PathTracerMISIntegrator;
use crate::integrators::path_tracer_nee::PathTracerNEEIntegrator;

#[enum_dispatch(Integrator)]
#[derive(Debug, Clone)]
pub enum IntegratorType {
    Normals(NormalsIntegrator),
    AmbientOcclusion(AmbientOcclusionIntegrator),
    PathTracerMats(PathTracerMatsIntegrator),
    PathTracerNEE(PathTracerNEEIntegrator),
    PathTracerMIS(PathTracerMISIntegrator),
}

pub fn create_integrator(m: &Map<String, Value>) -> IntegratorType {
    let Some(integrator_json) = m.get("integrator") else {
        println!("No integrator mentioned : using PathTracerMatsIntegrator");
        return IntegratorType::from(PathTracerMatsIntegrator::new(64));
    };

    let sampler_type = integrator_json
        .get("type")
        .expect("no integrator type")
        .as_str()
        .expect("could not get integrator type");

    match sampler_type {
        "normals" => IntegratorType::from(NormalsIntegrator {}),
        "ao" => IntegratorType::from(AmbientOcclusionIntegrator {}),
        "path_tracer_mats" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 64);
            IntegratorType::from(PathTracerMatsIntegrator::new(max_bounces))
        }
        "path_tracer_nee" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 64);
            IntegratorType::from(PathTracerNEEIntegrator::new(max_bounces))
        }
        "path_tracer_mis" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 64);
            IntegratorType::from(PathTracerMISIntegrator::new(max_bounces))
        }
        _ => {
            unimplemented!("Sampler type {}", sampler_type);
        }
    }
}
