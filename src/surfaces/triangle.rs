use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use crate::transform::Transform;
use nalgebra::{Vector2, Vector3};
use std::rc::Rc;
extern crate nalgebra_glm as glm;
use crate::aabb::Aabb;
use glm::{Vec2, Vec3};

pub struct Mesh {}

pub struct Triangle {
    pub mesh: Mesh,
    pub face_idx: usize,
}

// impl Surface for Triangle {
//     fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
//         // compute ray intersection (and ray parameter), continue if not hit
//         // put ray into sphere frame
//         let ray_transformed = self.transform.inverse().ray(ray);

//         if ray_transformed.direction.z == 0.0 {
//             return None;
//         };

//         let t = -ray_transformed.origin.z / ray_transformed.direction.z;
//         let mut p = ray_transformed.at(t);

//         if self.size.x < p.x.abs() || self.size.y < p.y.abs() {
//             return None;
//         }

//         // check if computed param is within ray.mint and ray.maxt
//         if t < ray_transformed.mint || t > ray_transformed.maxt {
//             return None;
//         }

//         // project hitpoint onto plane to reduce floating-point error
//         p.z = 0.0;

//         let n = glm::normalize(&self.transform.normal(&Vector3::z()));
//         let uv = 0.5 * p.xy().component_div(&self.size).add_scalar(1.0);
//         let uv = glm::clamp(&uv, 0.000001, 0.999999);

//         // if hit, set intersection record values
//         let hit = HitInfo {
//             t: t,
//             p: self.transform.point(&p),
//             gn: n,
//             sn: n,
//             uv: uv,
//             mat: Rc::clone(&self.material),
//         };
//         Some(hit)
//     }

//     fn bounds(&self) -> Aabb {
//         self.transform.aabb(&self.local_bounds())
//     }
// }

/// Ray-Triangle intersection
///
/// I use the Moller-Trumbore algorithm
/// 
/// # Arguments
/// * `ray` - Input Ray
/// * `v0` - Triangle vertices
/// * `v1` - Triangle vertices
/// * `v2` - Triangle vertices
/// * `n0` - Optional per vertex normal
/// * `n1` - Optional per vertex normal
/// * `n2` - Optional per vertex normal
/// * `t0` - Optional per vertex texture coordinates
/// * `t1` - Optional per vertex texture coordinates
/// * `t2` - Optional per vertex texture coordinates
/// * `material` - Triangle Materisl
pub fn single_triangle_intersect(
    ray: &Ray,
    v0: &Vec3,
    v1: &Vec3,
    v2: &Vec3,
    n0: &Option<Vec3>,
    n1: &Option<Vec3>,
    n2: &Option<Vec3>,
    t0: &Option<Vec2>,
    t1: &Option<Vec2>,
    t2: &Option<Vec2>,
    material: Rc<dyn Material>,
) -> Option<HitInfo> {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = glm::cross(&ray.direction, &edge2);
    let det = glm::dot(&edge1, &h);

    // check if the ray is parallel
    if det.abs() < 0.0000001 {
        return None;
    }

    // barycentric coordinate
    let inv_det = 1.0 / det;
    let s = ray.origin - v0;
    let u = glm::dot(&s, &h) * inv_det;

    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q = glm::cross(&s, &edge1);
    let v = glm::dot(&ray.direction, &q) * inv_det;

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // hit time
    let t = inv_det * glm::dot(&edge2, &q);
    if t < ray.mint || t > ray.maxt {
        return None;
    }

    // geometric normal
    let gn = glm::normalize(&glm::cross(&edge1, &edge2));

    // shading normal
    let sn = if n0.is_some() && n1.is_some() && n2.is_some() {
        //  barycentric interpolation of the per-vertex normals
        glm::normalize(&((1.0 - u - v) * (n0.unwrap()) + u * (n1.unwrap()) + v * (n2.unwrap())))
    } else {
        // We don't have per-vertex normals - just use the geometric normal
        gn
    };

    // vertex texture coordinates
    // Vec2f interpolated_uv;
    let uv = if t0.is_some() && t1.is_some() && t2.is_some() {
        // Do we have per-vertex texture coordinates available?
        //  barycentric interpolation of the per-vertex texture coordinates
        (1.0 - u - v) * (t0.unwrap()) + u * (t1.unwrap()) + v * (t2.unwrap())
    } else {
        // We don't have per-vertex texture coordinates - just use the geometric normal
        Vector2::new(u, v)
    };

    let hit = HitInfo {
        t: t,
        p: ray.at(t),
        gn: gn,
        sn: sn,
        uv: uv,
        mat: material,
    };
    Some(hit)
}

#[cfg(test)]
mod tests {
    use nalgebra::{Vector2, Vector3};

    use crate::surfaces::triangle::single_triangle_intersect;
    use crate::{materials::factory::create_material, ray::Ray};
    use assert_approx_eq::assert_approx_eq;
    use serde_json::json;

    #[test]
    fn triangle_intersection() {
        // Setup test data
        let v0 = Vector3::new(-2.0, -5.0, -1.0);
        let v1 = Vector3::new(1.0, 3.0, 1.0);
        let v2 = Vector3::new(2.0, -2.0, 3.0);

        let n0 = Some(Vector3::new(0.0, 0.707106, 0.707106));
        let n1 = Some(Vector3::new(0.666666, 0.333333, 0.666666));
        let n2 = Some(Vector3::new(0.0, -0.447213, -0.894427));

        let t0: Option<Vector2<f32>> = None;
        let t1: Option<Vector2<f32>> = None;
        let t2: Option<Vector2<f32>> = None;

        let ray = Ray::new(Vector3::new(1.0, -1.0, -5.0), Vector3::new(0.0, 0.20, 0.50));

        let material_json = json!({"type": "lambertian", "albedo": 1.0});
        let material = create_material(material_json);

        // run function
        if let Some(hit) =
            single_triangle_intersect(&ray, &v0, &v1, &v2, &n0, &n1, &n2, &t0, &t1, &t2, material)
        {
            // verify computed results
            let correct_t = 12.520326;
            let correct_p = Vector3::new(1.0, 1.504065, 1.260162);
            let correct_gn = Vector3::new(0.744073, -0.114473, -0.658218);
            let correct_sn = Vector3::new(0.762482, 0.317441, 0.563784);

            assert_approx_eq!(correct_t, hit.t, 1e-4);
            for i in 0..3 {
                assert_approx_eq!(correct_p[i], hit.p[i], 1e-4);
                assert_approx_eq!(correct_gn[i], hit.gn[i], 1e-4);
                assert_approx_eq!(correct_sn[i], hit.sn[i], 1e-4);
            }
        } else {
            assert!(false, "did not hit")
        }
    }
}
