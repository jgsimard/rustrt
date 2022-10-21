use crate::materials::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::sampling::{sample_hemisphere_cosine_power, sample_hemisphere_cosine_power_pdf};
use crate::surfaces::surface::{HitInfo, ScatterRecord};
use crate::textures::texture::{Texture, TextureType};
use crate::utils::reflect;
extern crate nalgebra_glm as glm;
use glm::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Phong {
    pub albedo: TextureType,
    pub exponent: f32
}

impl Material for Phong {
    fn scatter(&self, _r_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
        None
    }

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vec3> {
        None
    }
    fn is_emissive(&self) -> bool {
        false
    }

    fn eval(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> Vec3 {
        self.albedo.value(hit).unwrap() * self.pdf(wi, scattered, hit)
    }

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: &glm::Vec2) -> Option<ScatterRecord> {
        let mirror_dir = glm::normalize(&reflect(wi, &hit.gn));
        let uvw = ONB::build_from_w(&mirror_dir);
        let srec = ScatterRecord {
            attenuation: self.albedo.value(hit).unwrap(),
            wo: uvw.local(&sample_hemisphere_cosine_power(self.exponent, rv)),
            is_specular: false,
        };
        if glm::dot(&srec.wo, &hit.gn) >= 0.0{
            return Some(srec);
        }
        None
    }

    fn pdf(&self, _wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32 {
        let mirror_dir = glm::normalize(&reflect(scattered, &hit.gn));
        let cosine =  f32::max(glm::dot(&glm::normalize(scattered), &mirror_dir), 0.0);
        let pdf = sample_hemisphere_cosine_power_pdf(self.exponent , cosine);
        let final_pdf = if glm::dot(scattered, &hit.gn) >= 0.0 {pdf} else {0.0};
        return final_pdf;
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::{MaterialTest, SampleTest};
    use serde_json::json;


    #[test]
    fn phong_monte_carlo() {
        let v = json!({
            "type": "sample_material",
            "material": {
                "type": "phong",
                "albedo": 1.0,
                "exponent": 2
            },
            "normal": [
                0, 0, 1
            ],
            "name": "phong"
        });

        let mut test = MaterialTest::new(v);
        test.run();
    }

    #[test]
    fn phong_rotated_monte_carlo() {
        let v = json!({
            "type": "sample_material",
            "material": {
                "type": "phong",
                "albedo": 1.0,
                "exponent": 2
            },
            "normal": [
                0.25, 0.5, 1.0
            ],
            "name": "phong-rotated"
        });

        let mut test = MaterialTest::new(v);
        test.run();
    }
}
