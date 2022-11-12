extern crate nalgebra_glm as glm;
use glm::Vec3;
use rand::Rng;

use crate::integrators::integrator::Integrator;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::samplers::sampler::{Sampler, SamplerType};
use crate::scene::Scene;
use crate::surfaces::surface::Surface;

#[derive(Debug, Clone)]
pub struct AmbientOcclusionIntegrator;

impl Integrator for AmbientOcclusionIntegrator {
    fn li(&self, scene: &Scene, sampler: &SamplerType, rng: &mut impl Rng, ray: &Ray) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            let rv = sampler.next2f(rng);
            if let Some(srec) = hit.mat.sample(&ray.direction, &hit, &rv) {
                let shadow_ray = Ray::new(hit.p, srec.wo);
                // if shadow ray doesnt hit anything return white
                if scene.intersect(&shadow_ray).is_none() {
                    return Vec3::new(1.0, 1.0, 1.0);
                }
            }
        }
        Vec3::zeros()
    }
}
