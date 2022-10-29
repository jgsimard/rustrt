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

// iterative version
impl Integrator for PathTracerNEEIntegrator {
    fn li(&self, scene: &Scene, sampler: &mut SamplerType, ray_: &Ray, _depth: i32) -> Vec3 {
        const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let mut radiance = Vec3::zeros();
        let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut ray = Ray::new(ray_.origin, ray_.direction);

        for _ in 0..self.max_bounces {
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
        _ => {
            unimplemented!("Sampler type {}", sampler_type);
        }
    }
}
