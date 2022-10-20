use crate::materials::material::Material;
use crate::surfaces::surface::HitInfo;
extern crate nalgebra_glm as glm;
use crate::ray::Ray;
use crate::textures::texture::{Texture, TextureType};
use crate::utils::{luminance, reflect, reflectance, refract};
use crate::surfaces::surface::ScatterRecord;
use glm::Vec3;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
pub struct Dielectric {
    pub ior: TextureType,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
        let front_face = glm::dot(&hit.gn, &r_in.direction) < 0.0;
        let ior = luminance(&self.ior.value(hit).unwrap());
        let (normal, ratio_index_of_refraction) = if front_face {
            (hit.sn, 1.0 / ior)
        } else {
            (-hit.sn, ior)
        };

        let unit_direction = glm::normalize(&r_in.direction);

        let cos_theta = glm::dot(&((-1.0) * unit_direction), &normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let total_intern_reflection = ratio_index_of_refraction * sin_theta >= 1.0;

        let mut rng = rand::thread_rng();
        let will_reflect = rng.gen::<f32>() < reflectance(cos_theta, ratio_index_of_refraction);

        let direction = if total_intern_reflection || will_reflect {
            reflect(&unit_direction, &normal)
        } else {
            refract(&unit_direction, &normal, ratio_index_of_refraction)
        };

        let scattered = Ray::new(hit.p, direction);

        Some((Vec3::new(1.0, 1.0, 1.0), scattered))
    }

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vec3> {
        None
    }

    fn is_emissive(&self) -> bool {
        false
    }

    fn eval(&self,wi: &Vec3,scattered: &Vec3,hit: &HitInfo) -> Vec3 {
        Vec3::zeros()
    }

    fn sample(&self,wi: &Vec3,hit: &HitInfo,rv: &glm::Vec2) -> Option<(ScatterRecord,bool)> {
        None
    }

    fn pdf(&self,wi: &Vec3,scattered: &Vec3,hit: &HitInfo) -> f32 {
        0.0
    }
}
