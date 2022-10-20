use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, ScatterRecord};
extern crate nalgebra_glm as glm;
use crate::onb::ONB;
use crate::sampling::{sample_hemisphere, sample_hemisphere_pdf};
use glm::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct DiffuseLight {
    pub emit: Vec3,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _hit: &HitInfo) -> Option<(Vec3, Ray)> {
        None
    }

    fn emmitted(&self, ray: &Ray, hit: &HitInfo) -> Option<Vec3> {
        // only emit from the normal-facing side
        if glm::dot(&ray.direction, &hit.sn) > 0.0 {
            Some(Vec3::zeros())
        } else {
            Some(self.emit)
        }
    }

    fn is_emissive(&self) -> bool {
        true
    }

    fn eval(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> Vec3 {
        let emited_color = if glm::dot(&wi, &hit.sn) > 0.0 {
            Vec3::zeros()
        } else {
            self.emit
        };
        emited_color * self.pdf(wi, scattered, hit)
    }

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: &glm::Vec2) -> Option<(ScatterRecord, bool)> {
        let uvw = ONB::build_from_w(&hit.gn);
        let srec = ScatterRecord {
            attenuation: Vec3::zeros(), // FIXME
            wo: uvw.local(&sample_hemisphere(rv)),
            is_specular: false,
        };
        Some((srec, true))
    }

    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32 {
        sample_hemisphere_pdf(wi)
    }
}
