use crate::core::onb::Onb;
use crate::core::ray::Ray;
use crate::core::sampling::{sample_hemisphere, sample_hemisphere_pdf};
use crate::core::utils::read_v_or_f;
use crate::materials::Material;
use crate::surfaces::{HitInfo, ScatterRecord};

use nalgebra_glm::{dot, Vec2, Vec3};
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct DiffuseLight {
    emit: Vec3,
}

impl DiffuseLight {
    pub fn new(v: &Value) -> DiffuseLight {
        let emit = read_v_or_f(v, "emit");
        DiffuseLight { emit }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _hit: &HitInfo) -> Option<(Vec3, Ray)> {
        None
    }

    fn emmitted(&self, ray: &Ray, hit: &HitInfo) -> Option<Vec3> {
        // only emit from the normal-facing side
        if dot(&ray.direction, &hit.sn) > 0.0 {
            Some(Vec3::zeros())
        } else {
            Some(self.emit)
        }
    }

    fn is_emissive(&self) -> bool {
        true
    }

    fn eval(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> Vec3 {
        let emited_color = if dot(wi, &hit.sn) > 0.0 {
            Vec3::zeros()
        } else {
            self.emit
        };
        // emited_color
        emited_color * self.pdf(wi, scattered, hit)
    }

    fn sample(&self, _wi: &Vec3, hit: &HitInfo, rv: &Vec2) -> Option<ScatterRecord> {
        let uvw = Onb::build_from_w(&hit.gn);
        let srec = ScatterRecord {
            attenuation: Vec3::zeros(), // FIXME
            wo: uvw.local(&sample_hemisphere(rv)),
            is_specular: false,
        };
        Some(srec)
    }

    fn pdf(&self, wi: &Vec3, _scattered: &Vec3, _hit: &HitInfo) -> f32 {
        sample_hemisphere_pdf(wi)
    }
}
