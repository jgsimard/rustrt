mod blinn_phong;
mod dielectric;
mod diffuse_light;
mod fresnel_blend;
mod lambertian;
mod metal;
mod phong;

use enum_dispatch::enum_dispatch;
use nalgebra_glm::{Vec2, Vec3};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::core::ray::Ray;
use crate::core::utils::Factory;
use crate::surfaces::{HitInfo, ScatterRecord};

#[enum_dispatch]
pub trait Material {
    /// Compute the scattered direction scattered at a surface hitpoint.
    /// The base Material does not scatter any light, so it simply returns false.
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)>;

    /// Compute the amount of emitted light at the surface hitpoint.
    /// The base Material class does not emit light, so it simply returns black.
    fn emmitted(&self, ray: &Ray, hit: &HitInfo) -> Option<Vec3>;

    /// Return whether or not this Material is emissive.
    ///
    /// This is primarily used to create a global list of emitters for sampling.
    fn is_emissive(&self) -> bool;

    /// Evaluate the material response for the given pair of directions.
    ///
    /// For non-specular materials, this should be the BSDF multiplied by the
    /// cosine foreshortening term.
    /// Specular contributions should be excluded.
    fn eval(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> Vec3;

    /// Sample a scattered direction at the surface hitpoint \p hit.
    ///
    /// If it is not possible to evaluate the pdf of the material (e.g.\ it is
    /// specular or unknown), then set \c `srec.is_specular` to true, and populate
    /// \c srec.wo and \c srec.attenuation just like we did previously in the
    /// #scatter() function. This allows you to fall back to the way we did
    /// things with the #scatter() function, i.e.\ bypassing #pdf()
    /// evaluations needed for explicit Monte Carlo integration in your
    /// #Integrator, but this also precludes the use of MIS or mixture sampling
    /// since the pdf is unknown.
    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: Vec2) -> Option<ScatterRecord>;

    /// Compute the probability density that #sample() will generate \c scattered (given \c wi).
    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32;
}

use crate::materials::blinn_phong::BlinnPhong;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::fresnel_blend::FresnelBlend;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::materials::phong::Phong;

#[enum_dispatch(Material)]
#[derive(Debug, PartialEq, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
    DiffuseLight(DiffuseLight),
    FresnelBlend(FresnelBlend),
    Phong(Phong),
    BlinnPhong(BlinnPhong),
}

pub struct MaterialFactory {
    pub materials: HashMap<String, Arc<MaterialType>>,
}

impl MaterialFactory {
    pub fn new() -> MaterialFactory {
        MaterialFactory {
            materials: HashMap::new(),
        }
    }

    pub fn create_material(&self, v: &Value) -> Arc<MaterialType> {
        let type_material = v
            .get("type")
            .expect("material should have a type")
            .as_str()
            .expect("material type should be a string");

        let material = match type_material {
            "lambertian" => MaterialType::Lambertian(Lambertian::new(v)),
            "metal" => MaterialType::Metal(Metal::new(v)),
            "dielectric" => MaterialType::Dielectric(Dielectric::new(v)),
            "diffuse_light" => MaterialType::DiffuseLight(DiffuseLight::new(v)),
            "fresnel_blend" => MaterialType::FresnelBlend(FresnelBlend::new(v, self)),
            "phong" => MaterialType::Phong(Phong::new(v)),
            "blinn_phong" => MaterialType::BlinnPhong(BlinnPhong::new(v)),
            _ => unimplemented!("The material type '{}' ", type_material),
        };

        Arc::new(material)
    }
}

impl Factory<Arc<MaterialType>> for MaterialFactory {
    fn make(&mut self, v: &Value) -> Option<Vec<Arc<MaterialType>>> {
        let m = v.as_object().unwrap();
        let name = m
            .get("name")
            .expect("Feature doesnt have name")
            .to_string()
            .trim_matches('"')
            .to_string();
        let material = self.create_material(v);
        self.materials.insert(name, material.clone());
        Some(vec![material])
    }
}
