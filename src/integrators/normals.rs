extern crate nalgebra_glm as glm;
use glm::Vec3;
use rand::Rng;

use crate::integrators::integrator::Integrator;
use crate::ray::Ray;
use crate::samplers::sampler::SamplerType;
use crate::scene::Scene;
use crate::surfaces::surface::Surface;

#[derive(Debug, Clone)]
pub struct NormalsIntegrator;

impl Integrator for NormalsIntegrator {
    fn li(&self, scene: &Scene, _sampler: &SamplerType, _rng: &mut impl Rng, ray: &Ray) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            glm::abs(&hit.sn)
        } else {
            Vec3::zeros()
        }
    }
}
