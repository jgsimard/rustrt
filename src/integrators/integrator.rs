extern crate nalgebra_glm as glm;

use crate::materials::material::Material;
use crate::ray::Ray;
use crate::samplers::sampler::{Sampler, SamplerType};
use crate::scene::Scene;
use crate::surfaces::surface::Surface;
use crate::utils::read_or;
use enum_dispatch::enum_dispatch;
use glm::Vec3;

use serde_json::Value;

#[enum_dispatch]
pub trait Integrator {
    /// Sample the incident radiance along a ray
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray: &Ray, depth: i32) -> Vec3;

    /// To retrofit the code
    fn is_integrator(&self) -> bool {
        true
    }
}

#[enum_dispatch(Integrator)]
#[derive(Debug, Clone)]
pub enum IntegratorType {
    NotAnIntegrator,
    NormalsIntegrator,
    AmbientOcclusionIntegrator,
    PathTracerMatsIntegrator,
    PathTracerNEEIntegrator,
    PathTracerMISIntegrator,
}

#[derive(Debug, Clone)]
pub struct NotAnIntegrator;

impl Integrator for NotAnIntegrator {
    fn li(&self, _scene: &Scene, _sampler: &mut SamplerType, _ray: &Ray, _depth: i32) -> Vec3 {
        unimplemented!()
    }
    fn is_integrator(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct NormalsIntegrator;

impl Integrator for NormalsIntegrator {
    fn li(&self, scene: &Scene, _sampler: &mut SamplerType, ray: &Ray, _depth: i32) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            glm::abs(&hit.sn)
        } else {
            Vec3::zeros()
        }
    }
}

#[derive(Debug, Clone)]
pub struct AmbientOcclusionIntegrator;

impl Integrator for AmbientOcclusionIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray: &Ray, _depth: i32) -> Vec3 {
        if let Some(hit) = scene.intersect(ray) {
            let rv = sampler.next2f();
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

#[derive(Debug, Clone)]
pub struct PathTracerMatsIntegrator {
    max_bounces: i32,
}

// impl Integrator for PathTracerMatsIntegrator {
//     fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray: &Ray, depth: i32) -> Vec3 {
//         const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);

//         if let Some(hit) = scene.intersect(ray) {
//             let emitted = hit.mat.emmitted(ray, &hit).unwrap_or(BLACK);
//             if depth < self.max_bounces {
//                 let rv = sampler.next2f();
//                 if let Some(srec) = hit.mat.sample(&ray.direction, &hit, &rv) {
//                     let new_ray = Ray::new(hit.p, srec.wo);
//                     let recusive_li = self.li(scene, sampler, &new_ray, depth + 1);

//                     // RTIOW materials : no pdf
//                     if srec.is_specular {
//                         return emitted + srec.attenuation.component_mul(&recusive_li);
//                     } else {
//                         let attenuation = hit.mat.eval(&ray.direction, &srec.wo, &hit)
//                             / hit.mat.pdf(&ray.direction, &srec.wo, &hit);
//                         return emitted + attenuation.component_mul(&recusive_li);
//                     }
//                 }
//             }
//             return emitted;
//         } else {
//             return scene.background;
//         }
//     }
// }

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

/// Next Event Integrator
#[derive(Debug, Clone)]
pub struct PathTracerNEEIntegrator {
    max_bounces: i32,
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
                if let Some((emit_rec, v)) = scene.emitters.sample(&hit.p, &rv_light) {
                    // visibility
                    let visibility_ray = Ray::new(hit.p, emit_rec.wi);
                    if let Some(visibility_hit) = scene.intersect(&visibility_ray) {
                        let light_visible = (visibility_hit.t - emit_rec.hit.t).abs() < 1e-5;
                        if light_visible {
                            let select_probability = scene.emitters.pdf(&hit.p, &emit_rec.wi);
                            let mut light = hit.mat.eval(&ray.direction, &emit_rec.wi, &hit)
                                / (select_probability * emit_rec.pdf);
                            light = light.component_mul(&v);
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
                    hit.mat.eval(&ray.direction, &srec.wo, &hit) / hit.mat.pdf(&ray.direction, &srec.wo, &hit)
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

#[derive(Debug, Clone)]
pub struct PathTracerMISIntegrator {
    max_bounces: i32,
}

fn power_heuristic(pdf1: f32, pdf2: f32, power: f32) -> (f32, f32) {
    let pdf1_pow = pdf1.powf(power);
    let pdf2_pow = pdf2.powf(power);
    let den = pdf1_pow + pdf2_pow;
    return (pdf1_pow / den, pdf2_pow / den);
}

impl Integrator for PathTracerMISIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray_: &Ray, _depth: i32) -> Vec3 {
        const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = Ray::new(ray_.origin, ray_.direction);

        for _ in 0..=self.max_bounces {
            if let Some(hit) = scene.intersect(&ray) {
                let emitted = hit.mat.emmitted(&ray, &hit).unwrap_or(BLACK);

                // sample material
                let rv_mat = sampler.next2f();
                let srec = hit.mat.sample(&ray.direction, &hit, &rv_mat);
                if srec.is_none() {
                    break;
                }
                let srec = srec.unwrap();

                // sample light
                let rv_light = sampler.next2f();
                let light_sample = scene.emitters.sample(&hit.p, &rv_light);
                if light_sample.is_none() {
                    break;
                }
                let (emit_rec, v) = light_sample.unwrap();

                // mixture weight
                let pdf_mat = hit.mat.pdf(&ray.direction, &srec.wo, &hit);
                let select_probability = scene.emitters.pdf(&hit.p, &emit_rec.wi);
                let pdf_light = select_probability * emit_rec.pdf;

                let (weight_mat, weight_light) = power_heuristic(pdf_mat, pdf_light, 2.0);

                // light contibution
                let visibility_ray = Ray::new(hit.p, emit_rec.wi);
                if let Some(visibility_hit) = scene.intersect(&visibility_ray) {
                    let light_visible = (visibility_hit.t - emit_rec.hit.t).abs() < 1e-5;
                    if light_visible {
                        let mut light =
                            hit.mat.eval(&ray.direction, &emit_rec.wi, &hit) / pdf_light;
                        light = light.component_mul(&v);
                        light = light.component_mul(&attenuation);

                        radiance += light * weight_light;
                    }
                }

                // emitted contibution
                radiance += emitted.component_mul(&attenuation);

                // update for next bounce
                let mut a = if srec.is_specular {
                    srec.attenuation
                } else {
                    hit.mat.eval(&ray.direction, &srec.wo, &hit) / pdf_mat
                };
                a *= weight_mat;
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

pub fn create_integrator(v: &Value) -> IntegratorType {
    let m = v.as_object().unwrap();
    if !m.contains_key("integrator") {
        return IntegratorType::from(NotAnIntegrator {});
    }
    let integrator_json = v.get("integrator").unwrap();
    let sampler_type = integrator_json
        .get("type")
        .expect("no integrator type")
        .as_str()
        .expect("could not get integrator type");

    match sampler_type {
        "normals" => IntegratorType::from(NormalsIntegrator {}),
        "ao" => IntegratorType::from(AmbientOcclusionIntegrator {}),
        "path_tracer_mats" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 1);
            IntegratorType::from(PathTracerMatsIntegrator { max_bounces })
        }
        "path_tracer_nee" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 1);
            IntegratorType::from(PathTracerNEEIntegrator { max_bounces })
        }
        "path_tracer_mis" => {
            let max_bounces = read_or(integrator_json, "max_bounces", 1);
            IntegratorType::from(PathTracerMISIntegrator { max_bounces })
        }
        _ => {
            unimplemented!("Sampler type {}", sampler_type);
        }
    }
}
