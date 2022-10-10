use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use crate::transform::Transform;
use nalgebra::{Vector2, Vector3};
use std::rc::Rc;
extern crate nalgebra_glm as glm;

pub struct Quad {
    pub size: Vector2<f32>,
    pub transform: Transform,
    pub material: Rc<dyn Material>,
}

impl Surface for Quad {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // compute ray intersection (and ray parameter), continue if not hit
        // put ray into sphere frame
        let ray_transformed = self.transform.inverse().ray(ray);

        if ray_transformed.direction.z == 0.0 {
            return None;
        };

        let t = -ray_transformed.origin.z / ray_transformed.direction.z;
        let mut p = ray_transformed.at(t);

        if self.size.x < p.x.abs() || self.size.y < p.y.abs() {
            return None;
        }

        // check if computed param is within ray.mint and ray.maxt
        if t < ray_transformed.mint || t > ray_transformed.maxt {
            return None;
        }

        // project hitpoint onto plane to reduce floating-point error
        p.z = 0.0;

        let n = glm::normalize(&self.transform.normal(&Vector3::z()));
        let uv = 0.5 * p.xy().component_div(&self.size).add_scalar(1.0);
        let uv = glm::clamp(&uv, 0.000001, 0.999999);

        // if hit, set intersection record values
        let hit = HitInfo {
            t: t,
            p: self.transform.point(&p),
            gn: n,
            sn: n,
            uv: uv,
            mat: Rc::clone(&self.material),
        };
        Some(hit)
    }
}