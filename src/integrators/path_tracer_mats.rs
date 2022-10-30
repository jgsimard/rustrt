extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::integrators::integrator::Integrator;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::samplers::sampler::{Sampler, SamplerType};
use crate::scene::Scene;
use crate::surfaces::surface::Surface;

#[derive(Debug, Clone)]
pub struct PathTracerMatsIntegrator {
    pub max_bounces: i32,
}

// iterative version
impl Integrator for PathTracerMatsIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray_: &Ray, _depth: i32) -> Vec3 {
        const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = Ray::new(ray_.origin, ray_.direction);

        for _ in 0..=self.max_bounces {
            if let Some(hit) = scene.intersect(&ray) {
                let emitted = hit.mat.emmitted(&ray, &hit).unwrap_or(BLACK);
                let rv = sampler.next2f();
                if let Some(srec) = hit.mat.sample(&ray.direction, &hit, &rv) {
                    let a = if srec.is_specular {
                        srec.attenuation
                    } else {
                        hit.mat.eval(&ray.direction, &srec.wo, &hit)
                            / hit.mat.pdf(&ray.direction, &srec.wo, &hit)
                    };

                    radiance += emitted.component_mul(&attenuation);
                    attenuation = attenuation.component_mul(&a);

                    // update the ray for the next bounce
                    ray.origin = hit.p;
                    ray.direction = srec.wo;
                } else {
                    break;
                }
            } else {
                return radiance + scene.background.component_mul(&attenuation);
            }
        }
        return radiance;
    }
}
