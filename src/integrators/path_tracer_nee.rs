use nalgebra_glm::Vec3;
use rand::Rng;

use crate::core::ray::Ray;
use crate::core::scene::Scene;
use crate::integrators::Integrator;
use crate::materials::Material;
use crate::samplers::{Sampler, SamplerType};
use crate::surfaces::Surface;

/// Next Event Estimation Integrator
#[derive(Debug, Clone)]
pub struct PathTracerNEEIntegrator {
    max_bounces: i32,
}

impl PathTracerNEEIntegrator {
    pub fn new(max_bounces: i32) -> PathTracerNEEIntegrator {
        PathTracerNEEIntegrator { max_bounces }
    }
}

impl Integrator for PathTracerNEEIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, rng: &mut impl Rng, ray: &Ray) -> Vec3 {
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = ray.clone();

        for _ in 0..self.max_bounces {
            // find next intersection
            let Some(hit) = scene.intersect(&ray) else {
                return radiance + scene.background.component_mul(&attenuation);
            };

            // add emitted light contribution
            if let Some(emitted) = hit.mat.emmitted(&ray, &hit) {
                radiance += emitted.component_mul(&attenuation);
            }

            // sample material
            let rv_mat = sampler.next2f(rng);
            let Some(srec) = hit.mat.sample(&ray.direction, &hit, rv_mat) else {break;};

            if !srec.is_specular {
                // no need to sample light for specular materials
                let rv_light = sampler.next2f(rng);
                if let Some(emit_rec) =
                    scene
                        .emitters
                        .sample_from_group(&hit.p, rv_light, sampler.next1f(rng))
                {
                    // visibility
                    let visibility_ray = Ray::new(hit.p, emit_rec.wi);
                    if let Some(visibility_hit) = scene.intersect(&visibility_ray) {
                        let light_visible = (visibility_hit.t - emit_rec.hit.t).abs() < 1e-5;
                        if light_visible {
                            let select_probability = scene.emitters.pdf(&hit.p, &emit_rec.wi);
                            let light_pdf = select_probability * emit_rec.pdf;
                            let light =
                                hit.mat.eval(&ray.direction, &emit_rec.wi, &hit) / light_pdf;
                            let light = light.component_mul(&emit_rec.emitted);
                            let light = light.component_mul(&attenuation);
                            radiance += light;
                        }
                    }
                }
            }

            // update for next bounce
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
