use nalgebra_glm::Vec3;
use rand::Rng;

use crate::core::ray::Ray;
use crate::core::scene::Scene;
use crate::integrators::Integrator;
use crate::materials::Material;
use crate::samplers::{Sampler, SamplerType};
use crate::surfaces::Surface;

#[derive(Debug, Clone)]
pub struct AmbientOcclusionIntegrator;

impl Integrator for AmbientOcclusionIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, rng: &mut impl Rng, ray: &Ray) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            let rv = sampler.next2f(rng);
            if let Some(srec) = hit.mat.sample(&ray.direction, &hit, rv) {
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
