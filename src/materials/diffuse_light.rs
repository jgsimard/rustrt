use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
extern crate nalgebra_glm as glm;
use glm::Vec3;

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
}
