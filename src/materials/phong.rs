use crate::core::onb::Onb;
use crate::core::ray::Ray;
use crate::core::sampling::{sample_hemisphere_cosine_power, sample_hemisphere_cosine_power_pdf};
use crate::core::utils::{read_or, reflect};
use crate::materials::Material;
use crate::surfaces::{HitInfo, ScatterRecord};
use crate::textures::{create_texture, Texture, TextureType};

use nalgebra_glm::{dot, normalize, Vec2, Vec3};
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Phong {
    albedo: TextureType,
    exponent: f32,
}

impl Phong {
    pub fn new(v: &Value) -> Phong {
        let albedo = create_texture(v, "albedo");
        let exponent = read_or(v, "exponent", 1.0);
        Phong { albedo, exponent }
    }
}

impl Material for Phong {
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

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: Vec2) -> Option<ScatterRecord> {
        let mirror_dir = normalize(&reflect(wi, &hit.gn));
        let uvw = Onb::build_from_w(&mirror_dir);
        let srec = ScatterRecord {
            attenuation: self.albedo.value(hit).unwrap(),
            wo: uvw.local(&sample_hemisphere_cosine_power(self.exponent, rv)),
            is_specular: false,
        };
        if dot(&srec.wo, &hit.gn) >= 0.0 {
            return Some(srec);
        }
        None
    }

    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32 {
        let mirror_dir = normalize(&reflect(wi, &hit.gn));
        let cosine = f32::max(dot(&normalize(scattered), &mirror_dir), 0.0);
        let pdf = sample_hemisphere_cosine_power_pdf(self.exponent, cosine);
        if dot(scattered, &hit.gn) >= 0.0 {
            pdf
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::sample_test::MaterialTest;
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

        let (test, mut parameters) = MaterialTest::new(&v);
        parameters.run(&test, 1.0, 1e-2);
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

        let (test, mut parameters) = MaterialTest::new(&v);
        parameters.run(&test, 0.945, 1e-3);
    }
}
