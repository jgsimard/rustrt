use std::rc::Rc;

use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
use crate::textures::texture::{Texture, TextureType};
use crate::utils::luminance;
use crate::utils::reflectance;
// use crate::utils::random_in_unit_sphere;
extern crate nalgebra_glm as glm;
use glm::Vec3;
use rand::Rng;

use super::material::MaterialType;

#[derive(Debug, PartialEq, Clone)]
pub struct FresnelBlend {
    pub ior: TextureType,
    pub refracted: Rc<MaterialType>,
    pub reflected: Rc<MaterialType>,
}

impl Material for FresnelBlend {
    fn scatter(&self, ray_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
        let interior_ior = luminance(&self.ior.value(hit).unwrap());
        let normal = if glm::dot(&hit.gn, &ray_in.direction) < 0.0 {
            hit.sn
        } else {
            -hit.sn
        };
        let cos_theta = f32::min(glm::dot(&(-ray_in.direction), &normal), 1.0);

        let mut rng = rand::thread_rng();
        let will_reflect = rng.gen::<f32>() < reflectance(cos_theta, interior_ior);

        if will_reflect {
            self.reflected.scatter(ray_in, hit)
        } else {
            self.refracted.scatter(ray_in, hit)
        }
    }

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vec3> {
        None
    }
    fn is_emissive(&self) -> bool {
        false
    }
}
