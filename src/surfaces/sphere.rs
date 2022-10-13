use crate::box3::Box3;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use crate::transform::Transform;
use nalgebra::{Vector2, Vector3};
use std::rc::Rc;

pub struct Sphere {
    pub radius: f32,
    pub transform: Transform,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            radius: radius,
            transform: Default::default(),
            material: material,
        }
    }

    pub fn local_bounds(&self) -> Box3 {
        Box3 {
            min: Vector3::new(-self.radius, -self.radius, -self.radius),
            max: Vector3::new(self.radius, self.radius, self.radius),
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

        let hit = HitInfo {
            t: root,
            p: p,
            gn: n,
            sn: n,
            uv: Vector2::new(0.0, 0.0),
            mat: Rc::clone(&self.material),
        };
        Some(hit)
    }

    fn bounds(&self) -> Box3 {
        self.transform.box3(&self.local_bounds())
    }
}
