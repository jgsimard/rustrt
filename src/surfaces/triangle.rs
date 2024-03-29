use nalgebra::{Vector2, Vector3};
use nalgebra_glm::{cross, dot, length, length2, normalize, Vec2, Vec3};
use serde_json::Value;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use crate::core::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::sampling::{sample_triangle, sample_triangle_pdf};
use crate::core::transform::Transform;
use crate::core::utils::{read, INTERSECTION_TEST};
use crate::materials::{Material, MaterialType};
use crate::surfaces::{EmitterRecord, HitInfo, Surface};
use crate::surfaces::{SurfaceFactory, SurfaceType};

#[derive(Debug, PartialEq, Clone)]
pub struct Mesh {
    /// Vertex positions
    vertex_positions: Vec<Vec3>,

    /// Vertex normals
    vertex_normals: Vec<Vec3>,

    /// Vertex texture coordinates
    uvs: Vec<Vec2>,

    /// Vertex indices per face (triangle)
    vertex_indices: Vec<Vector3<usize>>,

    /// Normal indices per face (triangle)
    normal_indices: Vec<Vector3<usize>>,

    /// Texture indices per face (triangle)
    texture_indices: Vec<Vector3<usize>>,

    /// One material index per face (triangle)
    material_indices: Vec<usize>,

    /// All materials in the mesh
    // materials: Vec<Rc<dyn Material>>,
    materials: Arc<MaterialType>, // TODO : change this if multiple materials !

    /// Transformation that the data has already been transformed by
    transform: Transform,

    /// The bounds, after transformation
    bbox: Aabb,
}

impl Mesh {
    pub fn read(v: &Value, sf: &SurfaceFactory) -> Vec<SurfaceType> {
        let transform = Transform::read(v);
        let filename: String = read(v, "filename");

        let obj = tobj::load_obj(filename, &tobj::OFFLINE_RENDERING_LOAD_OPTIONS);
        let (models, _) = obj.expect("Failed to load OBJ file");
        let mut output = Vec::new();
        for model in models {
            let mesh = &model.mesh;
            let vs: Vec<Vec3> = mesh
                .positions
                .chunks(3)
                .map(|p| transform.point(&Vec3::new(p[0], p[1], p[2])))
                .collect();

            let mut aabb = Aabb::new();
            for vertex in &vs {
                aabb.enclose_point(vertex);
            }

            let ns: Vec<Vec3> = mesh
                .normals
                .chunks(3)
                .map(|p| Vec3::new(p[0], p[1], p[2]))
                .collect();

            let uvs: Vec<Vec2> = mesh
                .texcoords
                .chunks(2)
                .map(|p| Vec2::new(p[0], p[1]))
                .collect();

            let vertex_indices: Vec<Vector3<usize>> = mesh
                .indices
                .chunks(3)
                .map(|p| Vector3::new(p[0] as usize, p[1] as usize, p[2] as usize))
                .collect();

            let normal_indices: Vec<Vector3<usize>> = mesh
                .normal_indices
                .chunks(3)
                .map(|p| Vector3::new(p[0] as usize, p[1] as usize, p[2] as usize))
                .collect();

            let texture_indices: Vec<Vector3<usize>> = mesh
                .texcoord_indices
                .chunks(3)
                .map(|p| Vector3::new(p[0] as usize, p[1] as usize, p[2] as usize))
                .collect();

            assert_eq!(mesh.positions.len() % 3, 0);

            let m = v.as_object().unwrap();
            let material = sf.get_material(m);

            let n_triangles = vertex_indices.len();
            let my_mesh = Mesh {
                vertex_positions: vs,
                vertex_normals: ns,
                uvs,
                vertex_indices,
                normal_indices,
                texture_indices,
                material_indices: Vec::new(),
                materials: material,
                transform: transform.clone(),
                bbox: aabb,
            };

            let rc_mesh = Arc::new(my_mesh);

            let mut triangles = (0..n_triangles)
                .map(|i| {
                    SurfaceType::Triangle(Triangle {
                        mesh: rc_mesh.clone(),
                        face_idx: i,
                    })
                })
                .collect();
            output.append(&mut triangles);
        }
        output
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Triangle {
    mesh: Arc<Mesh>,
    face_idx: usize,
}

impl Triangle {
    pub fn new(v: &Value, sf: &SurfaceFactory) -> Triangle {
        let m = v.as_object().unwrap();
        let transform = Transform::read(v);
        let material = sf.get_material(m);

        assert!(
            m.contains_key("positions"),
            "Triangle should have 'positions'"
        );
        let pos = read::<Vec<Vec3>>(v, "positions");

        let mut aabb = Aabb::new();
        for vertex in &pos {
            aabb.enclose_point(vertex);
        }

        let (normals, normal_indices) = if m.contains_key("normals") {
            (read::<Vec<Vec3>>(v, "normals"), vec![Vector3::new(0, 1, 2)])
        } else {
            println!("no normals in triangle");
            (Vec::new(), Vec::new())
        };

        let (uvs, texture_indices) = if m.contains_key("uvs") {
            (read::<Vec<Vec2>>(v, "uvs"), vec![Vector3::new(0, 1, 2)])
        } else {
            println!("no texture in triangle");
            (Vec::new(), Vec::new())
        };

        let mesh = Mesh {
            vertex_positions: pos,
            vertex_normals: normals,
            uvs,
            vertex_indices: vec![Vector3::new(0, 1, 2)],
            normal_indices,
            texture_indices,
            material_indices: Vec::new(),
            materials: material,
            transform,
            bbox: aabb,
        };

        Triangle {
            mesh: Arc::new(mesh),
            face_idx: 0,
        }
    }
}

impl Triangle {
    /// convenience function to access the i-th vertex (i must be 0, 1, or 2)
    fn vertex(&self, i: usize) -> Vec3 {
        self.mesh.vertex_positions[self.mesh.vertex_indices[self.face_idx][i]]
    }
}

impl Surface for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        INTERSECTION_TEST.fetch_add(1, Ordering::SeqCst);

        // vertices
        let iv = self.mesh.vertex_indices[self.face_idx];

        let v0 = self.mesh.vertex_positions[iv.x];
        let v1 = self.mesh.vertex_positions[iv.y];
        let v2 = self.mesh.vertex_positions[iv.z];

        // shading normals
        let mut n0: Option<Vec3> = None;
        let mut n1: Option<Vec3> = None;
        let mut n2: Option<Vec3> = None;
        if self.mesh.normal_indices.len() > self.face_idx {
            let in_ = self.mesh.normal_indices[self.face_idx];
            n0.replace(self.mesh.vertex_normals[in_.x]);
            n1.replace(self.mesh.vertex_normals[in_.y]);
            n2.replace(self.mesh.vertex_normals[in_.z]);
        }

        // texture coordinates
        let mut t0: Option<Vector2<f32>> = None;
        let mut t1: Option<Vector2<f32>> = None;
        let mut t2: Option<Vector2<f32>> = None;
        if self.mesh.texture_indices.len() > self.face_idx {
            let it = self.mesh.texture_indices[self.face_idx];

            t0.replace(self.mesh.uvs[it.x]);
            t1.replace(self.mesh.uvs[it.y]);
            t2.replace(self.mesh.uvs[it.z]);
        }
        let material = self.mesh.materials.clone();
        single_triangle_intersect(ray, &v0, &v1, &v2, &n0, &n1, &n2, &t0, &t1, &t2, material)
    }

    fn bounds(&self) -> Aabb {
        // all mesh vertices have already been transformed to world space,
        // so just bound the triangle vertices
        let mut aabb = Aabb::new();
        aabb.enclose_point(&self.vertex(0));
        aabb.enclose_point(&self.vertex(1));
        aabb.enclose_point(&self.vertex(2));

        // if the triangle lies in an axis-aligned plane, expand the box a bit
        let diag = aabb.diagonal();
        for i in 0..3 {
            if diag[i] < 1e-4 {
                aabb.min[i] -= 5e-5;
                aabb.max[i] += 5e-5;
            }
        }
        aabb
    }

    fn sample(&self, origin: &Vec3, rv: Vec2) -> Option<EmitterRecord> {
        let v0 = self.vertex(0);
        let v1 = self.vertex(1);
        let v2 = self.vertex(2);

        let p = sample_triangle(&v0, &v1, &v2, rv);
        let wi = p - origin;
        let distance2 = length2(&wi);
        let t = f32::sqrt(distance2);
        let normal = self.mesh.transform.normal(&Vec3::z());

        let pdf = sample_triangle_pdf(&v0, &v1, &v2);
        let cosine = f32::abs(dot(&wi, &normal));
        let geometry_factor = distance2 / cosine;

        let pdf = geometry_factor * pdf;

        let hit = HitInfo {
            t,
            p,
            mat: self.mesh.materials.clone(),
            gn: normal,
            sn: normal,
            uv: Vec2::zeros(),
        };

        let emitted = self
            .mesh
            .materials
            .emmitted(&Ray::new(*origin, wi), &hit)
            .unwrap_or_default();

        let erec = EmitterRecord {
            o: *origin,
            wi,
            pdf,
            hit,
            emitted,
        };

        Some(erec)
    }

    fn pdf(&self, o: &Vec3, dir: &Vec3) -> f32 {
        if let Some(hit) = self.intersect(&Ray::new(*o, *dir)) {
            let v0 = self.vertex(0);
            let v1 = self.vertex(1);
            let v2 = self.vertex(2);

            let pdf = sample_triangle_pdf(&v0, &v1, &v2);

            let distance2 = hit.t * hit.t * length2(dir);
            let cosine = f32::abs(dot(dir, &hit.gn) / length(dir));
            let geometry_factor = distance2 / cosine;

            return geometry_factor * pdf;
        }
        0.0
    }

    // TODO : change this if multiple materials !
    fn is_emissive(&self) -> bool {
        self.mesh.materials.is_emissive()
    }
}

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
#[allow(clippy::too_many_arguments)]
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
    material: Arc<MaterialType>,
) -> Option<HitInfo> {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let h = cross(&ray.direction, &edge2);
    let det = dot(&edge1, &h);

    // check if the ray is parallel
    if det.abs() < 1e-10 {
        return None;
    }

    // barycentric coordinate
    let inv_det = 1.0 / det;
    let s = ray.origin - v0;
    let u = dot(&s, &h) * inv_det;

    if !(0.0..=1.0).contains(&u) {
        return None;
    }

    let q = cross(&s, &edge1);
    let v = dot(&ray.direction, &q) * inv_det;

    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // hit time
    let t = inv_det * dot(&edge2, &q);
    if t < ray.min_t || t > ray.max_t {
        return None;
    }

    // geometric normal
    let gn = normalize(&cross(&edge1, &edge2));

    // shading normal
    let sn = if n0.is_some() && n1.is_some() && n2.is_some() {
        //  barycentric interpolation of the per-vertex normals
        normalize(&((1.0 - u - v) * (n0.unwrap()) + u * (n1.unwrap()) + v * (n2.unwrap())))
    } else {
        // We don't have per-vertex normals - just use the geometric normal
        gn
    };

    // vertex texture coordinates
    let uv = if t0.is_some() && t1.is_some() && t2.is_some() {
        // Do we have per-vertex texture coordinates available?
        //  barycentric interpolation of the per-vertex texture coordinates
        (1.0 - u - v) * (t0.unwrap()) + u * (t1.unwrap()) + v * (t2.unwrap())
    } else {
        // We don't have per-vertex texture coordinates - just use the geometric normal
        Vector2::new(u, v)
    };

    let hit = HitInfo {
        t,
        p: ray.at(t),
        gn,
        sn,
        uv,
        mat: material,
    };
    Some(hit)
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use nalgebra_glm::{Vec2, Vec3};
    use serde_json::json;

    use crate::core::ray::Ray;
    use crate::materials::MaterialFactory;
    use crate::surfaces::triangle::single_triangle_intersect;

    #[test]
    fn ray_triangle_intersection() {
        // Setup test data
        let v0 = Vec3::new(-2.0, -5.0, -1.0);
        let v1 = Vec3::new(1.0, 3.0, 1.0);
        let v2 = Vec3::new(2.0, -2.0, 3.0);

        let n0 = Some(Vec3::new(
            0.0,
            std::f32::consts::FRAC_1_SQRT_2,
            std::f32::consts::FRAC_1_SQRT_2,
        ));
        let n1 = Some(Vec3::new(2. / 3., 1. / 3., 2. / 3.));
        let n2 = Some(Vec3::new(0.0, -0.447_213, -0.894_427));

        let t0: Option<Vec2> = None;
        let t1: Option<Vec2> = None;
        let t2: Option<Vec2> = None;

        let ray = Ray::new(Vec3::new(1.0, -1.0, -5.0), Vec3::new(0.0, 0.20, 0.50));

        let material_json = json!({"type": "lambertian", "albedo": 1.0});
        let mf = MaterialFactory::new();
        let material = mf.create_material(&material_json);

        // run function
        if let Some(hit) =
            single_triangle_intersect(&ray, &v0, &v1, &v2, &n0, &n1, &n2, &t0, &t1, &t2, material)
        {
            // verify computed results
            let correct_t = 12.520_326;
            let correct_p = Vec3::new(1.0, 1.504_065, 1.260_162);
            let correct_gn = Vec3::new(0.744_073, -0.114_473, -0.658_218);
            let correct_sn = Vec3::new(0.762_482, 0.317_441, 0.563_784);

            assert_abs_diff_eq!(correct_t, hit.t, epsilon = 1e-5);
            assert_abs_diff_eq!(correct_p, hit.p, epsilon = 1e-5);
            assert_abs_diff_eq!(correct_gn, hit.gn, epsilon = 1e-5);
            assert_abs_diff_eq!(correct_sn, hit.sn, epsilon = 1e-5);
        } else {
            panic!("did not hit")
        }
    }

    use crate::tests::sample_test::SurfaceTest;
    #[test]
    fn triangle_monte_carlo() {
        let v = json!({
            "type": "sample_surface",
            "name": "triangle",
            "surface": {
                "type": "triangle",
                "positions": [
                    [
                        -0.5, 0.2, -1.0
                    ],
                    [
                        0.5, 0.375, -1.0
                    ],
                    [
                        -0.5, 0.2, 1.0
                    ]
                ],
                "material": {
                    "type": "lambertian",
                    "albedo": 1.0
                }
            }
        });

        let (test, mut parameters) = SurfaceTest::new(&v);
        parameters.run(&test, 1.0, 1e-2);
    }
}
