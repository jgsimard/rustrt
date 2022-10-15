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

#[cfg(test)]
mod tests {
    use nalgebra::{Vector2, Vector3};
    use serde_json::json;
    use std::rc::Rc;

    use crate::materials::factory::create_material;
    use crate::ray::Ray;
    use crate::surfaces::surface::HitInfo;

    #[test]
    fn test_metal() {
        let surface_color = Vector3::new(1.0, 0.25, 0.25);

        // And now let's create a slightly shiny metal surface
        let metal_json = json!({
            "type": "metal",
            "albedo": surface_color,
            "roughness": 0.3
        });
        let metal_material = create_material(metal_json);

        // Let's create a fictitious hitpoint
        let surface_point = Vector3::new(1.0, 2.0, 0.0);
        let normal = Vector3::new(1.0, 2.0, -1.0).normalize();
        let hit = HitInfo {
            t: 0.0,
            p: surface_point,
            uv: Vector2::new(0.0, 0.0),
            gn: normal,
            sn: normal,
            mat: Rc::clone(&metal_material),
        };

        // And a fictitious ray
        let ray = Ray::new(Vector3::new(2.0, 3.0, -1.0), Vector3::new(-1.0, -1.0, 1.0));

        println!("Testing metal scatter");
        if let Some((metal_attenuation, metal_scattered)) = metal_material.scatter(&ray, &hit) {
            let correct_origin = surface_point.clone();
            let correct_attenuation = surface_color.clone();
            // let correct_direction = Vector3::new(2.697650e-01, 9.322242e-01, -2.421507e-01);

            approx::assert_abs_diff_eq!(correct_origin, metal_scattered.origin, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_attenuation, metal_attenuation, epsilon = 1e-5);
            // approx::assert_abs_diff_eq!(correct_direction, metal_scattered.direction, epsilon = 1e-5);
        } else {
            println!("Metal scatter incorrect! Scattering should have been successful\n");
        }
    }
}
