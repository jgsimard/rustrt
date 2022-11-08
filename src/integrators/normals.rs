extern crate nalgebra_glm as glm;
use glm::Vec3;
use std::sync::{Arc, Mutex};

use crate::integrators::integrator::Integrator;
use crate::ray::Ray;
use crate::samplers::sampler::SamplerType;
use crate::scene::Scene;
use crate::surfaces::surface::Surface;

#[derive(Debug, Clone)]
pub struct NormalsIntegrator;

impl Integrator for NormalsIntegrator {
    fn li(&self, scene: &Scene, _sampler: &mut SamplerType, ray: &Ray) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            glm::abs(&hit.sn)
        } else {
            Vec3::zeros()
        }
    }
}
