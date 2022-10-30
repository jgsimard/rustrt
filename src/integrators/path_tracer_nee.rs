extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::integrators::integrator::Integrator;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::samplers::sampler::{Sampler, SamplerType};
use crate::scene::Scene;
use crate::surfaces::surface::Surface;

/// Next Event Estimation Integrator
#[derive(Debug, Clone)]
pub struct PathTracerNEEIntegrator {
    pub max_bounces: i32,
}

impl Integrator for PathTracerNEEIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray_: &Ray, _depth: i32) -> Vec3 {
        const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = Ray::new(ray_.origin, ray_.direction);

        for _ in 0..self.max_bounces {
            if let Some(hit) = scene.intersect(&ray) {
                let emitted = hit.mat.emmitted(&ray, &hit).unwrap_or(BLACK);

                let rv_light = sampler.next2f();
                if let Some(emit_rec) =
                    scene
                        .emitters
                        .sample_from_group(&hit.p, &rv_light, sampler.next1f())
                {
                    // visibility
                    let visibility_ray = Ray::new(hit.p, emit_rec.wi);
                    if let Some(visibility_hit) = scene.intersect(&visibility_ray) {
                        let light_visible = (visibility_hit.t - emit_rec.hit.t).abs() < 1e-5;
                        if light_visible {
                            let select_probability = scene.emitters.pdf(&hit.p, &emit_rec.wi);
                            let mut light = hit.mat.eval(&ray.direction, &emit_rec.wi, &hit)
                                / (select_probability * emit_rec.pdf);
                            light = light.component_mul(&emit_rec.emitted);
                            light = light.component_mul(&attenuation);
                            radiance += light;
                        }
                    }
                }

                radiance += emitted.component_mul(&attenuation);

                // sample material
                let rv_mat = sampler.next2f();
                let srec = hit.mat.sample(&ray.direction, &hit, &rv_mat);
                if srec.is_none() {
                    break;
                }
                let srec = srec.unwrap();

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
            } else {
                return radiance + scene.background.component_mul(&attenuation);
            }
        }
        return radiance;
    }
}