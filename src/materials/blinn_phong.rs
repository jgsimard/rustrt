use crate::materials::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::sampling::{sample_hemisphere_cosine_power, sample_hemisphere_cosine_power_pdf};
use crate::surfaces::surface::{HitInfo, ScatterRecord};
use crate::textures::texture::{create_texture, Texture, TextureType};
use crate::utils::{read_or, reflect};
use serde_json::Value;

extern crate nalgebra_glm as glm;
use glm::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct BlinnPhong {
    albedo: TextureType,
    exponent: f32,
}

impl Material for BlinnPhong {
    fn scatter(&self, _r_in: &Ray, _hit: &HitInfo) -> Option<(Vec3, Ray)> {
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
        let uvw = ONB::build_from_w(&hit.gn);
        let normal = uvw.local(&sample_hemisphere_cosine_power(self.exponent, rv));

        let mirror_dir = glm::normalize(&reflect(wi, &normal));

        let srec = ScatterRecord {
            attenuation: self.albedo.value(hit).unwrap(),
            wo: mirror_dir,
            is_specular: false,
        };
        if glm::dot(&srec.wo, &hit.gn) >= 0.0 {
            return Some(srec);
        }
        None
    }

    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32 {
        let random_normal = glm::normalize(&(-glm::normalize(wi) + glm::normalize(scattered)));
        let cosine = f32::max(glm::dot(&random_normal, &hit.gn), 0.0);
        let normal_pdf = sample_hemisphere_cosine_power_pdf(self.exponent, cosine);
        let final_pdf = normal_pdf / (4.0 * glm::dot(&(-wi), &random_normal));
        let ouput_pdf = if glm::dot(scattered, &hit.gn) >= 0.0 {
            final_pdf
        } else {
            0.0
        };
        return ouput_pdf;
    }
}

impl BlinnPhong {
    pub fn new(v: &Value) -> BlinnPhong {
        let albedo = create_texture(v, "albedo");
        let exponent = read_or(v, "exponent", 1.0);
        BlinnPhong { albedo, exponent }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::sample_test::MaterialTest;
    use serde_json::json;

    #[test]
    fn blinn_phong_monte_carlo() {
        let v = json!({
            "type": "sample_material",
            "material": {
                "type": "blinn_phong",
                "albedo": 1.0,
                "exponent": 10
            },
            "normal": [
                0, 0, 1
            ],
            "name": "blinn_phong"
        });

        let (test, mut parameters) = MaterialTest::new(v);
        parameters.run(&test, 0.969, 1e-3);
    }

    #[test]
    fn blin_phong_rotated_monte_carlo() {
        let v = json!({
            "type": "sample_material",
            "material": {
                "type": "blinn_phong",
                "albedo": 1.0,
                "exponent": 10
            },
            "normal": [
                0.25, 0.5, 1.0
            ],
            "name": "blinn_phong-rotated"
        });

        let (test, mut parameters) = MaterialTest::new(v);
        parameters.run(&test, 0.909, 1e-3);
    }
}
