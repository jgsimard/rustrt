use nalgebra_glm::{Vec2, Vec3};
use rand::Rng;
use serde_json::Value;

use crate::core::ray::Ray;
use crate::core::sampling::sample_sphere;
use crate::core::utils::{luminance, reflect};
use crate::materials::Material;
use crate::surfaces::HitInfo;
use crate::surfaces::ScatterRecord;
use crate::textures::{create_texture, Texture, TextureType};

#[derive(Debug, PartialEq, Clone)]
pub struct Metal {
    albedo: TextureType,
    roughness: TextureType,
}

impl Metal {
    pub fn new(v: &Value) -> Metal {
        let albedo = create_texture(v, "albedo");
        let roughness = create_texture(v, "roughness");
        Metal { albedo, roughness }
    }

    fn _scatter(&self, r_in: &Ray, hit: &HitInfo, rv: &Vec2) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r_in.direction, &hit.sn);
        let roughness = luminance(&self.roughness.value(hit).unwrap());
        let scatter_direction = reflected + roughness * sample_sphere(rv);

        if scatter_direction.dot(&hit.sn) < 0.0 {
            return None;
        }
        let attenuation = self.albedo.value(hit).unwrap();
        let ray_out = Ray::new(hit.p, scatter_direction.normalize());

        Some((attenuation, ray_out))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<(Vec3, Ray)> {
        let mut rng = rand::thread_rng();
        let rv = Vec2::new(rng.gen(), rng.gen());
        self._scatter(ray, hit, &rv)
    }

    fn emmitted(&self, _ray: &Ray, _hit: &HitInfo) -> Option<Vec3> {
        None
    }

    fn is_emissive(&self) -> bool {
        false
    }

    fn eval(&self, _wi: &Vec3, _scattered: &Vec3, _hit: &HitInfo) -> Vec3 {
        Vec3::zeros()
    }

    fn sample(&self, wi: &Vec3, hit: &HitInfo, rv: &Vec2) -> Option<ScatterRecord> {
        let ray = Ray::new(hit.p - wi, *wi);
        let (attenuation, ray_out) = self._scatter(&ray, hit, rv)?;
        let srec = ScatterRecord {
            attenuation,
            wo: ray_out.direction,
            is_specular: true,
        };
        Some(srec)
    }

    fn pdf(&self, _wi: &Vec3, _scattered: &Vec3, _hit: &HitInfo) -> f32 {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use nalgebra_glm::{Vec2, Vec3};
    use serde_json::json;

    use crate::core::ray::Ray;
    use crate::materials::{Material, MaterialFactory};
    use crate::surfaces::HitInfo;

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
            mat: metal_material.clone(),
        };

        // And a fictitious ray
        let ray = Ray::new(Vec3::new(2.0, 3.0, -1.0), Vec3::new(-1.0, -1.0, 1.0));

        println!("Testing metal scatter");
        if let Some((metal_attenuation, metal_scattered)) = metal_material.scatter(&ray, &hit) {
            let correct_origin = surface_point;
            let correct_attenuation = surface_color;
            // let correct_direction = Vec3::new(2.697650e-01, 9.322242e-01, -2.421507e-01);

            approx::assert_abs_diff_eq!(correct_origin, metal_scattered.origin, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_attenuation, metal_attenuation, epsilon = 1e-5);
            // approx::assert_abs_diff_eq!(correct_direction, metal_scattered.direction, epsilon = 1e-5);
        } else {
            println!("Metal scatter incorrect! Scattering should have been successful\n");
        }
    }
}
