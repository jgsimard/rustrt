extern crate nalgebra_glm as glm;
use glm::Vec3;
use rand::Rng;

use crate::core::ray::Ray;
use crate::core::scene::Scene;
use crate::integrators::integrator::Integrator;
use crate::materials::material::Material;
use crate::samplers::sampler::{Sampler, SamplerType};
use crate::surfaces::surface::Surface;

#[derive(Debug, Clone)]
pub struct PathTracerMatsIntegrator {
    pub max_bounces: i32,
}

// iterative version
impl Integrator for PathTracerMatsIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, rng: &mut impl Rng, ray: &Ray) -> Vec3 {
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = ray.clone();

        for _ in 0..=self.max_bounces {
            // find next intersection
            let Some(hit) = scene.intersect(&ray) else {
                return radiance + scene.background.component_mul(&attenuation);
            };

            // sample next direction
            let rv = sampler.next2f(rng);
            let Some(srec) = hit.mat.sample(&ray.direction, &hit, &rv) else { break;};

            // add emitted light contribution
            if let Some(emitted) = hit.mat.emmitted(&ray, &hit) {
                radiance += emitted.component_mul(&attenuation);
            }

            // update attenuation
            let a = if srec.is_specular {
                srec.attenuation
            } else {
                hit.mat.eval(&ray.direction, &srec.wo, &hit)
                    / hit.mat.pdf(&ray.direction, &srec.wo, &hit)
            };
            attenuation = attenuation.component_mul(&a);

            // update the ray for the next bounce
            ray.origin = hit.p;
            ray.direction = srec.wo;
        }
        radiance
    }
}
