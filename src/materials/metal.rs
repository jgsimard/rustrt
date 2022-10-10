use nalgebra::Vector3;

use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
use crate::utils::{random_in_unit_sphere, reflect};

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub roughness: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vector3<f32>, Ray)> {
        let mut rng = rand::thread_rng();

        let reflected = reflect(&r_in.direction, &hit.sn);

        let scatter_direction =
            reflected + self.roughness * random_in_unit_sphere(&mut rng).normalize();

        if scatter_direction.dot(&hit.sn) < 0.0 {
            return None;
        }
        let attenuation = self.albedo;
        let ray_out = Ray::new(hit.p, scatter_direction.normalize());

        Some((attenuation, ray_out))
    }

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vector3<f32>> {
        None
    }

    fn is_emissive(&self) -> bool {
        false
    }
}
