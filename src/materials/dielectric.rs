use nalgebra_glm::{dot, normalize, Vec2, Vec3};
use rand::Rng;
use serde_json::Value;

use crate::core::ray::Ray;
use crate::core::utils::{luminance, reflect, reflectance, refract};
use crate::materials::Material;
use crate::surfaces::HitInfo;
use crate::surfaces::ScatterRecord;
use crate::textures::{create_texture, Texture, TextureType};

#[derive(Debug, PartialEq, Clone)]
pub struct Dielectric {
    ior: TextureType,
}

impl Dielectric {
    pub fn new(v: &Value) -> Dielectric {
        let ior = create_texture(v, "ior");
        Dielectric { ior }
    }

    fn _scatter(&self, ray: &Ray, hit: &HitInfo, rv: f32) -> Option<(Vec3, Ray)> {
        let front_face = dot(&hit.gn, &ray.direction) < 0.0;
        let ior = luminance(&self.ior.value(hit).unwrap());
        let (normal, ratio_index_of_refraction) = if front_face {
            (hit.sn, 1.0 / ior)
        } else {
            (-hit.sn, ior)
        };

        let unit_direction = normalize(&ray.direction);

        let cos_theta = dot(&((-1.0) * unit_direction), &normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let total_intern_reflection = ratio_index_of_refraction * sin_theta >= 1.0;

        let will_reflect = rv < reflectance(cos_theta, ratio_index_of_refraction);

        let direction = if total_intern_reflection || will_reflect {
            reflect(&unit_direction, &normal)
        } else {
            refract(&unit_direction, &normal, ratio_index_of_refraction)
        };

        let scattered = Ray::new(hit.p, direction);

        Some((Vec3::new(1.0, 1.0, 1.0), scattered))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();
        let rv = rng.gen();
        self._scatter(ray, hit, rv)
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

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: &Vec2) -> Option<ScatterRecord> {
        let ray = Ray::new(hit.p - wi, *wi);
        let (attenuation, ray_out) = self._scatter(&ray, hit, rv.x)?;
        let srec = ScatterRecord {
            attenuation,
            wo: ray_out.direction,
            is_specular: true,
        };
        Some(srec)
    }

    fn pdf(&self, _wi: &Vec3, _scattered: &Vec3, _hit: &HitInfo) -> f32 {
        0.0
    }
}
