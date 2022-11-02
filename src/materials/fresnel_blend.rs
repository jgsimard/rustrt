use std::rc::Rc;

use crate::materials::material::{Material, MaterialFactory};
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, ScatterRecord};
use crate::textures::texture::{create_texture, Texture, TextureType};
use crate::utils::{luminance, reflectance};

use serde_json::{from_value, Value};
extern crate nalgebra_glm as glm;
use glm::Vec3;
use rand::Rng;

use super::material::MaterialType;

#[derive(Debug, PartialEq, Clone)]
pub struct FresnelBlend {
    ior: TextureType,
    refracted: Rc<MaterialType>,
    reflected: Rc<MaterialType>,
}

impl FresnelBlend {
    pub fn new(v: &Value, mf: &MaterialFactory) -> FresnelBlend {
        let ior = create_texture(&v, "ior");
        let refracted_v = v.get("refr").unwrap().clone();
        let refracted = if refracted_v.is_string() {
            let refracted_name: String = from_value(refracted_v).unwrap();
            (*mf.materials
                .get(&refracted_name)
                .expect("doesnt contain refr"))
            .clone()
        } else if v.is_object() {
            mf.create_material(refracted_v)
        } else {
            panic!("NOOOOOO refr : {}", refracted_v);
        };

        let reflected_v = v.get("refl").expect("no refl").clone();
        let reflected = if reflected_v.is_string() {
            let reflected_name: String = from_value(reflected_v).unwrap();
            (*mf.materials
                .get(&reflected_name)
                .expect("doesnt contain refl"))
            .clone()
        } else if v.is_object() {
            mf.create_material(reflected_v)
        } else {
            panic!("NOOOOOO refl : {}", reflected_v);
        };
        FresnelBlend {
            ior: ior,
            refracted: refracted,
            reflected: reflected,
        }
    }
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
