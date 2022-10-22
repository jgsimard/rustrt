use crate::materials::material::{MaterialType, Material};
use crate::ray::Ray;
use crate::surfaces::surface::{EmitterRecord, HitInfo, Surface};
use crate::transform::Transform;
use std::rc::Rc;
extern crate nalgebra_glm as glm;
use crate::aabb::Aabb;
use glm::{Vec2, Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Quad {
    pub size: Vec2,
    pub transform: Transform,
    pub material: Rc<MaterialType>,
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

        let n = glm::normalize(&self.transform.normal(&Vec3::z()));
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

    fn bounds(&self) -> Aabb {
        self.transform.aabb(&self.local_bounds())
    }

    fn pdf(&self, o: &Vec3, dir: &Vec3) -> f32 {
        if let Some(hit) = self.intersect(&Ray::new(*o, *dir)) {
            let v0 = self.transform.vector(&Vec3::new(self.size.x, 0.0, 0.0));
            let v1 = self.transform.vector(&Vec3::new(0.0, self.size.y, 0.0));

            let area = 4.0 * glm::length(&glm::cross(&v0, &v1));
            let distance2 = hit.t * hit.t * glm::length2(&dir);
            let cosine = f32::abs(glm::dot(&dir, &hit.gn) / glm::length(&dir));
            let geometry_factor = distance2 / cosine;
            let pdf = 1.0 / area;

            return  geometry_factor * pdf;
        }
        return 0.0;
    } 

    fn sample(&self, o: &Vec3,rv: &Vec2) -> Option<(EmitterRecord,Vec3)> {
        let new_rv = (rv * 2.0).add_scalar(-1.0);
        let temp = new_rv.component_mul(&self.size);
        let raw_p = Vec3::new(temp.x, temp.y, 0.0);

        let p   = self.transform.point(&raw_p);
        let wi = p - o;
        let distance2 = glm::length2(&wi);
        let t = f32::sqrt(distance2);
        let normal = self.transform.normal(&Vec3::z());
        let wi = wi / t;



        let v0 = self.transform.vector(&Vec3::new(self.size.x, 0.0, 0.0));
        let v1 = self.transform.vector(&Vec3::new(0.0, self.size.y, 0.0));

        let area = 4.0 * glm::length(&glm::cross(&v0, &v1));
        // let distance2 = hit.t * hit.t * glm::length2(&dir);
        let cosine = f32::abs(glm::dot(&wi, &normal));
        let geometry_factor = distance2 / cosine;
        let pdf = 1.0 / area * geometry_factor;

        let hit = HitInfo {
            t: t,
            p: p,
            mat: self.material.clone(),
            gn: normal,
            sn: normal,
            uv: Vec2::zeros(),
        };

        let emitted = if let Some(e) = self.material.emmitted(&Ray::new(o.clone(), wi), &hit) {
            e / pdf
        } else {
            Vec3::zeros()
        };


        let erec = EmitterRecord {
            o: o.clone(),
            wi: wi,
            pdf: pdf,
            hit: hit,
        };

        Some((erec, emitted))
    }


    fn is_emissive(&self) -> bool {
        self.material.is_emissive()
    }
}

impl Quad {
    fn local_bounds(&self) -> Aabb {
        const EPS: f32 = 1e-4 as f32;
        let v = glm::vec3(self.size.x + EPS, self.size.y + EPS, EPS);
        Aabb { min: -v, max: v }
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::testing::SurfaceTest;

    #[test]
    fn quad_monte_carlo() {
        let v = json!({
            "type": "sample_surface",
            "name": "quad",
            "surface": {
                "type": "quad",
                "size": 1.0,
                "transform": {
                    "o": [
                        0, 0, 1
                    ],
                    "x": [
                        1, 0, 0
                    ],
                    "y": [0, 1, 1]
                },
                "material": {
                    "type": "lambertian",
                    "albedo": 1.0
                }
            }
        });

        let (test, mut parameters) = SurfaceTest::new(v);
        parameters.run(&test, 1.0, 1e-2);
    }
}