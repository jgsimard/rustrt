use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
use nalgebra::Vector3;
extern crate nalgebra_glm as glm;

pub struct DiffuseLight {
    pub emit: Vector3<f32>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _hit: &HitInfo) -> Option<(Vector3<f32>, Ray)> {
        None
    }

    fn emmitted(&self, ray: &Ray, hit: &HitInfo) -> Option<Vector3<f32>> {
        // only emit from the normal-facing side
        if glm::dot(&ray.direction, &hit.sn) > 0.0 {
            Some(Vector3::zeros())
        } else {
            Some(self.emit)
        }
    }

    fn is_emissive(&self) -> bool {
        true
    }
}
