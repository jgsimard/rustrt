use nalgebra_glm::{length, Vec2, Vec3};
use serde_json::Value;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::core::aabb::Aabb;
use crate::core::onb::Onb;
use crate::core::ray::Ray;
use crate::core::sampling::{sample_sphere_cap, sample_sphere_cap_pdf};
use crate::core::transform::Transform;
use crate::core::utils::{direction_to_spherical_uv, read_or, INTERSECTION_TEST};
use crate::materials::{Material, MaterialType};
use crate::surfaces::{EmitterRecord, HitInfo, Surface, SurfaceFactory};

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    transform: Transform,
    radius: f32,
    material: Arc<MaterialType>,
}

impl Sphere {
    pub fn local_bounds(&self) -> Aabb {
        Aabb {
            min: Vec3::new(-self.radius, -self.radius, -self.radius),
            max: Vec3::new(self.radius, self.radius, self.radius),
        }
    }
    pub fn new(v: &Value, sf: &SurfaceFactory) -> Sphere {
        let radius = read_or(v, "radius", 1.0);
        let transform = Transform::read(v);
        let material = sf.get_material(v.as_object().unwrap());

        Sphere {
            radius,
            transform,
            material,
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        INTERSECTION_TEST.fetch_add(1, Ordering::SeqCst);
        // put ray into sphere frame
        let ray_transformed = self.transform.inverse().ray(ray);

        let oc = ray_transformed.origin;

        let a = ray_transformed.direction.norm_squared();
        let half_b = oc.dot(&ray_transformed.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        // Find the nearest root that lies in the acceptable range
        let discriminant_sqrt = discriminant.sqrt();
        let mut root = (-half_b - discriminant_sqrt) / a;
        if root < ray_transformed.min_t || root > ray_transformed.max_t {
            root = (-half_b + discriminant_sqrt) / a;
            if root < ray_transformed.min_t || root > ray_transformed.max_t {
                return None;
            }
        }

        let p_sphere_frame = ray_transformed.at(root);
        // put point and normal back into the world frame
        let p = self.transform.point(&p_sphere_frame);
        let n = self.transform.normal(&(p_sphere_frame / self.radius));
        let uv = direction_to_spherical_uv(&p_sphere_frame);

        let hit = HitInfo {
            t: root,
            p,
            gn: n,
            sn: n,
            uv,
            mat: Arc::clone(&self.material),
        };
        Some(hit)
    }

    fn bounds(&self) -> Aabb {
        self.transform.aabb(&self.local_bounds())
    }

    fn sample(&self, o: &Vec3, rv: Vec2) -> Option<EmitterRecord> {
        let center = self.transform.point(&Vec3::zeros());
        let direction_centre: Vec3 = center - o;
        let dist = length(&(o - center));

        let radius = length(&self.transform.vector(&(Vec3::z()))) * self.radius;
        let cos_theta_max_from_p = f32::sqrt(dist * dist - radius * radius) / dist;

        if radius > dist {
            return None;
        }
        // sample from p
        let uvw = Onb::build_from_w(&direction_centre);
        let sample_direction = uvw.local(&sample_sphere_cap(rv, cos_theta_max_from_p));

        let sample_ray = Ray::new(*o, sample_direction);

        let hit = self.intersect(&sample_ray)?;
        let pdf = sample_sphere_cap_pdf(cos_theta_max_from_p, cos_theta_max_from_p);
        let emitted = self
            .material
            .emmitted(&sample_ray, &hit)
            .unwrap_or_default();

        let erec = EmitterRecord {
            o: *o,
            wi: sample_direction,
            pdf,
            hit,
            emitted,
        };

        Some(erec)
    }

    fn pdf(&self, o: &Vec3, dir: &Vec3) -> f32 {
        let test_ray = Ray::new(*o, *dir);
        if let Some(_hit) = self.intersect(&test_ray) {
            let center = self.transform.point(&Vec3::zeros());
            // let direction = center - o;
            let dist = length(&(o - center));
            let cos_theta_max = f32::sqrt(dist * dist - self.radius * self.radius) / dist;
            let pdf = sample_sphere_cap_pdf(cos_theta_max, cos_theta_max);
            return pdf;
        }
        0.0
    }

    fn is_emissive(&self) -> bool {
        self.material.is_emissive()
    }
}

#[cfg(test)]
mod tests {
    use nalgebra_glm::Vec3;

    use crate::core::ray::Ray;
    use crate::core::transform::Transform;
    use crate::materials::MaterialFactory;
    use crate::surfaces::Sphere;
    use crate::surfaces::Surface;

    use serde_json::json;

    extern crate approx;

    #[test]
    fn ray_sphere_intersection() {
        // Let's check if your implementation was correct:

        let lambert_json = json!({
            "type": "lambertian",
            "albedo": 1.0
        });
        let mf = MaterialFactory::new();
        let material = mf.create_material(&lambert_json);

        let test_sphere = Sphere {
            radius: 1.0,
            transform: Transform::default(),
            material: material.clone(),
        };

        println!("Testing untransformed sphere intersection");
        let test_ray = Ray::new(Vec3::new(-0.25, 0.5, 4.0), Vec3::new(0.0, 0.0, -1.0));
        // HitInfo hit;
        if let Some(hit) = test_sphere.intersect(&test_ray) {
            let correct_t = 3.170_844;
            let correct_p = Vec3::new(-0.25, 0.5, 0.829_156);
            let correct_n = Vec3::new(-0.25, 0.5, 0.829_156);

            approx::assert_abs_diff_eq!(correct_t, hit.t, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_p, hit.p, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_n, hit.sn, epsilon = 1e-5);
        } else {
            panic!("Sphere intersection incorrect! Should hit sphere");
        }

        // Now, let's check if you implemented sphere transforms correctly!
        let transform = Transform::axis_offset(
            &Vec3::new(2.0, 0.0, 0.0),  // x-axis
            &Vec3::new(0.0, 1.0, 0.0),  // y-axis
            &Vec3::new(0.0, 0.0, 0.5),  // z-axis
            &Vec3::new(0.0, 0.25, 5.0), // translation
        );
        let transformed_sphere = Sphere {
            radius: 1.0,
            transform,
            material,
        };
        let test_ray = Ray::new(Vec3::new(1.0, 0.5, 8.0), Vec3::new(0.0, 0.0, -1.0));

        println!("Testing transformed sphere intersection");
        if let Some(hit) = transformed_sphere.intersect(&test_ray) {
            let correct_t = 2.585_422;
            let correct_p = Vec3::new(1.0, 0.5, 5.41458);
            let correct_n = Vec3::new(0.147_442, 0.147_442, 0.978_019);

            approx::assert_abs_diff_eq!(correct_t, hit.t, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_p, hit.p, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_n, hit.sn, epsilon = 1e-5);
        } else {
            panic!("Transformed sphere intersection incorrect! Should hit sphere");
        }
    }

    use crate::tests::sample_test::SurfaceTest;

    #[test]
    fn sphere_monte_carlo() {
        let v = json!({
            "type": "sample_surface",
            "name": "sphere",
            "surface": {
                "type": "sphere",
                "radius": 3.0,
                "transform": {
                    "o": [0, 3.2, 0.4]
                },
                "material": {
                    "type": "lambertian",
                    "albedo": 1.0
                }
            }
        });

        let (test, mut parameters) = SurfaceTest::new(&v);
        parameters.run(&test, 1.0, 1e-3);
    }
}
