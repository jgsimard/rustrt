use nalgebra_glm::{dot, normalize, Vec2, Vec3};
use serde_json::Value;

use crate::core::onb::Onb;
use crate::core::ray::Ray;
use crate::core::sampling::{sample_hemisphere_cosine_power, sample_hemisphere_cosine_power_pdf};
use crate::core::utils::{read_or, reflect};
use crate::materials::Material;
use crate::surfaces::{HitInfo, ScatterRecord};
use crate::textures::{create_texture, Texture, TextureType};

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

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: Vec2) -> Option<ScatterRecord> {
        let uvw = Onb::build_from_w(&hit.gn);
        let normal = uvw.local(&sample_hemisphere_cosine_power(self.exponent, rv));

        let mirror_dir = normalize(&reflect(wi, &normal));

        if dot(&mirror_dir, &hit.gn) >= 0.0 {
            let srec = ScatterRecord {
                attenuation: self.albedo.value(hit).unwrap(),
                wo: mirror_dir,
                is_specular: false,
            };
            return Some(srec);
        }
        None
    }

    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32 {
        let random_normal = normalize(&(-normalize(wi) + normalize(scattered)));
        let cosine = f32::max(dot(&random_normal, &hit.gn), 0.0);
        let normal_pdf = sample_hemisphere_cosine_power_pdf(self.exponent, cosine);
        let final_pdf = normal_pdf / (4.0 * dot(&(-wi), &random_normal));
        if dot(scattered, &hit.gn) >= 0.0 {
            final_pdf
        } else {
            0.0
        }
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

        let (test, mut parameters) = MaterialTest::new(&v);
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

        let (test, mut parameters) = MaterialTest::new(&v);
        parameters.run(&test, 0.909, 1e-3);
    }
}
