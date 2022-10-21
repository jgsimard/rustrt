extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
use crate::surfaces::surface::ScatterRecord;
use crate::textures::texture::{Texture, TextureType};
use crate::utils::{luminance, random_in_unit_sphere, reflect};

#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: TextureType,
    pub roughness: TextureType,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();

        let reflected = reflect(&r_in.direction, &hit.sn);
        let roughness = luminance(&self.roughness.value(hit).unwrap());
        let rand_vec = random_in_unit_sphere(&mut rng).normalize();
        let scatter_direction = reflected + roughness * rand_vec;

        if scatter_direction.dot(&hit.sn) < 0.0 {
            return None;
        }
        let attenuation = self.albedo.value(hit).unwrap();
        let ray_out = Ray::new(hit.p, scatter_direction.normalize());

        Some((attenuation, ray_out))
    }

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vec3> {
        None
    }

    fn is_emissive(&self) -> bool {
        false
    }

    fn eval(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> Vec3 {
        Vec3::zeros()
    }

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: &glm::Vec2) -> Option<ScatterRecord> {
        None
    }

    fn pdf(&self, wi: &Vec3, scattered: &Vec3, hit: &HitInfo) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    extern crate nalgebra_glm as glm;
    use glm::{Vec2, Vec3};
    use serde_json::json;
    use std::rc::Rc;

    use crate::materials::factory::MaterialFactory;
    use crate::materials::material::Material;
    use crate::ray::Ray;
    use crate::surfaces::surface::HitInfo;

    #[test]
    fn test_metal() {
        let surface_color = Vec3::new(1.0, 0.25, 0.25);

        // And now let's create a slightly shiny metal surface
        let metal_json = json!({
            "type": "metal",
            "albedo": surface_color,
            "roughness": 0.3
        });
        let mf = MaterialFactory::new();
        let metal_material = mf.create_material(metal_json);

        // Let's create a fictitious hitpoint
        let surface_point = Vec3::new(1.0, 2.0, 0.0);
        let normal = Vec3::new(1.0, 2.0, -1.0).normalize();
        let hit = HitInfo {
            t: 0.0,
            p: surface_point,
            uv: Vec2::new(0.0, 0.0),
            gn: normal,
            sn: normal,
            mat: Rc::clone(&metal_material),
        };

        // And a fictitious ray
        let ray = Ray::new(Vec3::new(2.0, 3.0, -1.0), Vec3::new(-1.0, -1.0, 1.0));

        println!("Testing metal scatter");
        if let Some((metal_attenuation, metal_scattered)) = metal_material.scatter(&ray, &hit) {
            let correct_origin = surface_point.clone();
            let correct_attenuation = surface_color.clone();
            // let correct_direction = Vec3::new(2.697650e-01, 9.322242e-01, -2.421507e-01);

            approx::assert_abs_diff_eq!(correct_origin, metal_scattered.origin, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_attenuation, metal_attenuation, epsilon = 1e-5);
            // approx::assert_abs_diff_eq!(correct_direction, metal_scattered.direction, epsilon = 1e-5);
        } else {
            println!("Metal scatter incorrect! Scattering should have been successful\n");
        }
    }
}
