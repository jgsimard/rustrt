use std::rc::Rc;

use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, ScatterRecord};
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
    fn eval(&self, _wi: &Vec3, _scattered: &Vec3, _hit: &HitInfo) -> Vec3 {
        Vec3::zeros()
    }

    fn sample(&self, wi: &Vec3, hit: &HitInfo, _rv: &glm::Vec2) -> Option<ScatterRecord> {
        let ray = Ray::new(hit.p - wi, *wi);
        if let Some((attenuation, ray_out)) = self.scatter(&ray, hit) {
            let srec = ScatterRecord {
                attenuation: attenuation,
                wo: ray_out.direction,
                is_specular: true,
            };
            return Some(srec);
        }
        return None;
    }

    fn pdf(&self, _wi: &Vec3, _scattered: &Vec3, _hit: &HitInfo) -> f32 {
        1.0
    }
}
