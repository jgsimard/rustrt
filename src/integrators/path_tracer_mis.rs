use nalgebra_glm::Vec3;
use rand::Rng;

use crate::core::ray::Ray;
use crate::core::scene::Scene;
use crate::integrators::Integrator;
use crate::materials::Material;
use crate::samplers::{Sampler, SamplerType};
use crate::surfaces::Surface;

/// Multiple Importance Sampling Integrator
#[derive(Debug, Clone)]
pub struct PathTracerMISIntegrator {
    max_bounces: i32,
}

impl PathTracerMISIntegrator {
    pub fn new(max_bounces: i32) -> PathTracerMISIntegrator {
        PathTracerMISIntegrator { max_bounces }
    }
}

#[allow(unused)]
fn power_heuristic(pdf1: f32, pdf2: f32, power: f32) -> (f32, f32) {
    let pdf1_pow = pdf1.powf(power);
    let pdf2_pow = pdf2.powf(power);
    let den = pdf1_pow + pdf2_pow;
    (pdf1_pow / den, pdf2_pow / den)
}

#[allow(unused)]
fn balance_heuristic(pdf1: f32, pdf2: f32) -> (f32, f32) {
    let den = pdf1 + pdf2;
    (pdf1 / den, pdf2 / den)
}

impl Integrator for PathTracerMISIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, rng: &mut impl Rng, ray_: &Ray) -> Vec3 {
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = ray_.clone();
        let mut previous_weight_mat = 1.0;

        for bounce in 0..=self.max_bounces {
            // find next intersection hit point
            let Some(hit) = scene.intersect(&ray) else {
                return radiance + scene.background.component_mul(&attenuation);
            };

            // sample material
            let rv_mat = sampler.next2f(rng);
            let Some(srec) = hit.mat.sample(&ray.direction, &hit, rv_mat) else { break };

            // sample light
            let rv_light = sampler.next2f(rng);
            let Some(emit_rec) =
                scene
                    .emitters
                    .sample_from_group(&hit.p, rv_light, sampler.next1f(rng)) else { break };

            // mixture weight
            let pdf_mat = hit.mat.pdf(&ray.direction, &srec.wo, &hit);
            let select_probability = scene.emitters.pdf(&hit.p, &emit_rec.wi);
            let pdf_light = select_probability * emit_rec.pdf;

            let (weight_mat, weight_light) = if srec.is_specular {
                (1.0, 0.0)
            } else {
                power_heuristic(pdf_mat, pdf_light, 2.0)
            };

            // light contibution
            if !srec.is_specular {
                // no light samples from specular materials
                let visibility_ray = Ray::new(hit.p, emit_rec.wi);
                if let Some(visibility_hit) = scene.intersect(&visibility_ray) {
                    let light_visible = (visibility_hit.t - emit_rec.hit.t).abs() < 1e-5;
                    if light_visible {
                        let light = hit
                            .mat
                            .eval(&ray.direction, &emit_rec.wi, &hit)
                            .component_mul(&emit_rec.emitted)
                            .component_mul(&attenuation)
                            / pdf_light
                            * weight_light;
                        radiance += light;
                    }
                }
            }

            // emitted contibution
            if let Some(emitted) = hit.mat.emmitted(&ray, &hit) {
                if bounce == 0 || srec.is_specular {
                    radiance += emitted.component_mul(&attenuation);
                } else {
                    radiance += emitted.component_mul(&attenuation) * previous_weight_mat;
                }
            }

            // update for next bounce
            let mat_attenuation = if srec.is_specular {
                srec.attenuation
            } else {
                let mat_eval = hit.mat.eval(&ray.direction, &srec.wo, &hit);
                mat_eval / pdf_mat
            };

            attenuation = attenuation.component_mul(&mat_attenuation);

            // update the ray for the next bounce
            ray.origin = hit.p;
            ray.direction = srec.wo;
            previous_weight_mat = weight_mat;
        }
        radiance
    }
}
