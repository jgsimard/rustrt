use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::HitInfo;
use crate::utils::random_in_unit_sphere;
use nalgebra::Vector3;

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitInfo) -> Option<(Vector3<f32>, Ray)> {
        let mut rng = rand::thread_rng();
        let mut scatter_direction = hit.sn + random_in_unit_sphere(&mut rng).normalize();

        // Catch degenerate scatter direction
        const EPSILON: f32 = 1.0e-6;
        if scatter_direction.norm_squared() < EPSILON {
            scatter_direction = hit.sn;
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
    fn test_lambertian() {
        let surface_color = Vector3::new(1.0, 0.25, 0.25);

        let lambert_json = json!({
            "type": "lambertian",
            "albedo": surface_color
        });
        let lambert_material = create_material(lambert_json);

        // Let's create a fictitious hitpoint
        let surface_point = Vector3::new(1.0, 2.0, 0.0);
        let normal = Vector3::new(1.0, 2.0, -1.0).normalize();
        let hit = HitInfo {
            t: 0.0,
            p: surface_point,
            uv: Vector2::new(0.0, 0.0),
            gn: normal,
            sn: normal,
            mat: Rc::clone(&lambert_material),
        };

        // And a fictitious ray
        let ray = Ray::new(Vector3::new(2.0, 3.0, -1.0), Vector3::new(-1.0, -1.0, 1.0));

        // Now, let's test your implementation!
        if let Some((lambert_attenuation, lambert_scattered)) = lambert_material.scatter(&ray, &hit)
        {
            let correct_origin = surface_point.clone();
            let correct_attenuation = surface_color.clone();
            // let correct_direction = Vector3::new(1.206627e+00, 3.683379e-01, -8.104229e-01);

            approx::assert_abs_diff_eq!(correct_origin, lambert_scattered.origin, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_attenuation, lambert_attenuation, epsilon = 1e-5);
            // approx::assert_abs_diff_eq!(correct_direction, lambert_scattered.direction, epsilon = 1e-5);
        } else {
            println!("Lambert scatter incorrect! Scattering should have been successful\n");
        }
    }
}
