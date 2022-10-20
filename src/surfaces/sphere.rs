use crate::aabb::Aabb;
use crate::materials::material::MaterialType;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use crate::transform::Transform;
use crate::utils::direction_to_spherical_uv;
extern crate nalgebra_glm as glm;
use glm::Vec3;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub radius: f32,
    pub transform: Transform,
    pub material: Rc<MaterialType>,
}

impl Sphere {
    pub fn local_bounds(&self) -> Aabb {
        Aabb {
            min: Vec3::new(-self.radius, -self.radius, -self.radius),
            max: Vec3::new(self.radius, self.radius, self.radius),
        }
    }
}

impl Surface for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
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
        if root < ray_transformed.mint || root > ray_transformed.maxt {
            root = (-half_b + discriminant_sqrt) / a;
            if root < ray_transformed.mint || root > ray_transformed.maxt {
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
            p: p,
            gn: n,
            sn: n,
            uv: uv,
            mat: Rc::clone(&self.material),
        };
        Some(hit)
    }

    fn bounds(&self) -> Aabb {
        self.transform.aabb(&self.local_bounds())
    }
}

#[cfg(test)]
mod tests {
    extern crate nalgebra_glm as glm;
    use glm::Vec3;
    use std::rc::Rc;

    use crate::materials::factory::create_texture;
    use crate::materials::lambertian::Lambertian;
    use crate::materials::material::MaterialType;
    use crate::ray::Ray;
    use crate::surfaces::sphere::Sphere;
    use crate::surfaces::surface::Surface;
    use crate::transform::Transform;

    use serde_json::json;

    extern crate approx;

    #[test]
    fn test_ray_sphere_intersection() {
        // Let's check if your implementation was correct:
        let material: Rc<MaterialType> = Rc::new(MaterialType::from(Lambertian {
            albedo: create_texture(&json!({"albedo": 1.0}), "albedo"),
        }));
        let test_sphere = Sphere {
            radius: 1.0,
            transform: Default::default(),
            material: Rc::clone(&material),
        };

        println!("Testing untransformed sphere intersection");
        let test_ray = Ray::new(Vec3::new(-0.25, 0.5, 4.0), Vec3::new(0.0, 0.0, -1.0));
        // HitInfo hit;
        if let Some(hit) = test_sphere.intersect(&test_ray) {
            let correct_t = 3.170844;
            let correct_p = Vec3::new(-0.25, 0.5, 0.829156);
            let correct_n = Vec3::new(-0.25, 0.5, 0.829156);

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
            transform: transform,
            material: Rc::clone(&material),
        };
        let test_ray = Ray::new(Vec3::new(1.0, 0.5, 8.0), Vec3::new(0.0, 0.0, -1.0));

        println!("Testing transformed sphere intersection");
        if let Some(hit) = transformed_sphere.intersect(&test_ray) {
            let correct_t = 2.585422;
            let correct_p = Vec3::new(1.0, 0.5, 5.41458);
            let correct_n = Vec3::new(0.147442, 0.147442, 0.978019);

            approx::assert_abs_diff_eq!(correct_t, hit.t, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_p, hit.p, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_n, hit.sn, epsilon = 1e-5);
        } else {
            panic!("Transformed sphere intersection incorrect! Should hit sphere");
        }
    }
}
