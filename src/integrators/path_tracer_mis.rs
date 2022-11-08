extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::integrators::integrator::Integrator;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::samplers::sampler::{Sampler, SamplerType};
use crate::scene::Scene;
use crate::surfaces::surface::Surface;

/// Multiple Importance Sampling Integrator
#[derive(Debug, Clone)]
pub struct PathTracerMISIntegrator {
    pub max_bounces: i32,
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
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray_: &Ray) -> Vec3 {
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = ray_.clone();

        for _ in 0..=self.max_bounces {
            // find next intersection hit point
            let Some(hit) = scene.intersect(&ray) else {
                return radiance + scene.background.component_mul(&attenuation);
            };

            // sample material
            let rv_mat = sampler.next2f();
            let Some(srec) = hit.mat.sample(&ray.direction, &hit, &rv_mat) else { break };

            // sample light
            let rv_light = sampler.next2f();
            let Some(emit_rec) =
                scene
                    .emitters
                    .sample_from_group(&hit.p, rv_light, sampler.next1f()) else { break };

            // mixture weight
            let pdf_mat = hit.mat.pdf(&ray.direction, &srec.wo, &hit);
            let select_probability = scene.emitters.pdf(&hit.p, &emit_rec.wi);
            let pdf_light = select_probability * emit_rec.pdf;

            // let pdf_avg = (pdf_mat + pdf_light) / 2.0;

            // let (weight_mat, weight_light) = if srec.is_specular {
            //     (1.0, 1.0)
            //     // power_heuristic(1.0, pdf_light, 2.0)
            //     // balance_heuristic(1.0, pdf_light)
            // } else {
            //     // (1.0, 1.0)
            //     power_heuristic(pdf_mat, pdf_light, 2.0)
            //     // balance_heuristic(pdf_mat, pdf_light)
            // };

            // light contibution
            let visibility_ray = Ray::new(hit.p, emit_rec.wi);
            if let Some(visibility_hit) = scene.intersect(&visibility_ray) {
                let light_visible = (visibility_hit.t - emit_rec.hit.t).abs() < 1e-5;
                if light_visible {
                    let mat_eval = hit.mat.eval(&ray.direction, &emit_rec.wi, &hit);
                    let mut light = mat_eval / pdf_light;
                    // let mut light = mat_eval / pdf_avg;

                    light = light.component_mul(&emit_rec.emitted);
                    light = light.component_mul(&attenuation);
                    // light *= 0.5;
                    // light *= weight_light;

                    radiance += light;
                }
            }

            // emitted contibution
            if let Some(emitted) = hit.mat.emmitted(&ray, &hit) {
                radiance += emitted.component_mul(&attenuation);
            }

            // update for next bounce
            let mat_attenuation = if srec.is_specular {
                srec.attenuation
            } else {
                let mat_eval = hit.mat.eval(&ray.direction, &srec.wo, &hit);
                mat_eval / pdf_mat
                // mat_eval / pdf_avg
            };
            // mat_attenuation *= weight_mat;
            // mat_attenuation *= 0.5;
            attenuation = attenuation.component_mul(&mat_attenuation);

            // update the ray for the next bounce
            ray.origin = hit.p;
            ray.direction = srec.wo;
        }
        radiance
    }
}
