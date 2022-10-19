extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
use crate::utils::{random_in_unit_sphere, reflect};

#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub roughness: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
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

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vec3> {
        None
    }

    fn is_emissive(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    extern crate nalgebra_glm as glm;
    use glm::{Vec2, Vec3};
    use serde_json::json;
    use std::rc::Rc;

    use crate::materials::factory::create_material;
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
        let metal_material = create_material(metal_json);

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
