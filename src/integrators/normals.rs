use nalgebra_glm::{abs, Vec3};
use rand::Rng;

use crate::core::ray::Ray;
use crate::core::scene::Scene;
use crate::integrators::Integrator;
use crate::samplers::SamplerType;

#[derive(Debug, Clone)]
pub struct NormalsIntegrator;

impl Integrator for NormalsIntegrator {
    fn li(
        &self,
        scene: &Scene,
        _sampler: &mut SamplerType,
        _rng: &mut impl Rng,
        ray: &Ray,
    ) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            abs(&hit.sn)
        } else {
            Vec3::zeros()
        }
    }
}
