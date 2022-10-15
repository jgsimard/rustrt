use crate::aabb::Aabb;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use crate::transform::{parse_transform, Transform};

use nalgebra::{Vector2, Vector3};
use std::rc::Rc;
extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use serde_json::{from_value, Value};
use tobj;

pub struct Mesh {
    /// Vertex positions
    pub vs: Vec<Vec3>,

    /// Vertex normals
    pub ns: Vec<Vec3>,

    /// Vertex texture coordinates
    pub uvs: Vec<Vec2>,

    /// Vertex indices per face (triangle)
    pub Fv: Vec<Vector3<usize>>,

    /// Normal indices per face (triangle)
    pub Fn: Vec<Vector3<usize>>,

    /// Texture indices per face (triangle)
    pub Ft: Vec<Vector3<usize>>,

    /// One material index per face (triangle)
    pub Fm: Vec<usize>,

    /// All materials in the mesh
    // materials: Vec<Rc<dyn Material>>,
    pub materials: Rc<dyn Material>,
    
    /// Transformation that the data has already been transformed by
    pub transform: Transform,

    /// The bounds, after transformation
    pub bbox: Aabb,
}

impl Surface for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        unimplemented!()
    }

    fn bounds(&self) -> Aabb {
        self.bbox.clone()
    }
}

impl Mesh {
    fn add_to_vec(&self, sur: Rc<dyn Surface>, surfaces_vec: &mut Vec<Rc<dyn Surface>>){
        // surfaces_vec.push(sur);
        let n_triangles = self.Fv.len();
        let rc_mesh = Rc::new(*self);
        for i in 0..n_triangles{
            surfaces_vec.push(Rc::new(Triangle{mesh : rc_mesh.clone(), face_idx: i}));
        }
    }
    // pub fn new(json: &Value) {
    //     let filename: String = from_value(json["filename"].clone()).expect("no filename");

    //     let obj = tobj::load_obj(filename, &tobj::OFFLINE_RENDERING_LOAD_OPTIONS);
    //     // let obj = tobj::load_obj(filename, &tobj::GPU_LOAD_OPTIONS);
    //     // let load_options = tobj::LoadOptions {
    //     //     // single_index: true,
    //     //     ..Default::default()
    //     // };
    //     // let obj = tobj::load_obj(filename, &load_options);

    //     assert!(obj.is_ok());
    //     let (models, _) = obj.expect("Failed to load OBJ file");
    //     println!("# of models: {}", models.len());

    //     let mesh = &models[0].mesh;

    //     // for (i, m) in models.iter().enumerate() {
    //     //     let mesh = &m.mesh;

    //     println!("model.name = \'{}\'", models[0].name);
    //     println!("model.mesh.material_id = {:?}", mesh.material_id);

    //     println!("Size of model.face_arities: {}", mesh.face_arities.len());

    //     let mut next_face = 0;
    //     for f in 0..mesh.face_arities.len() {
    //         let end = next_face + mesh.face_arities[f] as usize;
    //         let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
    //         println!("    face[{}] = {:?}", f, face_indices);
    //         next_face = end;
    //     }

    //     println!("model.vertices: {}", mesh.positions.len() / 3);
    //     println!("model.vertex_color: {}", mesh.vertex_color.len() / 3);
    //     println!("model.normals: {}", mesh.normals.len() / 3);
    //     println!("model.texcoords: {}", mesh.texcoords.len() / 3);
    //     println!("model.indices: {}", mesh.indices.len() / 3);
    //     println!("model.face_arities: {}", mesh.face_arities.len() / 3);
    //     println!("model.texcoord_indices: {}", mesh.texcoord_indices.len() / 3);
    //     println!("model.material_id: {}", mesh.material_id.unwrap_or_default());

    //     let vs: Vec<Vec3> = mesh
    //         .positions
    //         .chunks(3)
    //         .map(|p| Vec3::new(p[0], p[1], p[2]))
    //         .collect();

    //     let ns: Vec<Vec3> = mesh
    //         .normals
    //         .chunks(3)
    //         .map(|p| Vec3::new(p[0], p[1], p[2]))
    //         .collect();

    //     let uvs: Vec<Vec2> = mesh
    //         .texcoords
    //         .chunks(2)
    //         .map(|p| Vec2::new(p[0], p[1]))
    //         .collect();

    //     let Fv: Vec<Vector3<usize>> = mesh
    //         .indices
    //         .chunks(3)
    //         .map(|p| Vector3::new(p[0] as usize, p[1] as usize, p[2] as usize))
    //         .collect();

    //     let Fn: Vec<Vector3<usize>> = mesh
    //         .normal_indices
    //         .chunks(3)
    //         .map(|p| Vector3::new(p[0] as usize, p[1] as usize, p[2] as usize))
    //         .collect();

    //     let Ft: Vec<Vector3<usize>> = mesh
    //         .texcoord_indices
    //         .chunks(3)
    //         .map(|p| Vector3::new(p[0] as usize, p[1] as usize, p[2] as usize))
    //         .collect();

    //     assert!(mesh.positions.len() % 3 == 0);

    //     let transform = if json.as_object().unwrap().contains_key("transform") {
    //         parse_transform(&json["transform"])
    //     } else {
    //         Default::default()
    //     };
    //     let n_triangles = Fv.len();

    //     let my_mesh = Mesh{
    //         vs: vs,
    //         ns: ns,
    //         uvs: uvs,
    //         Fv: Fv,
    //         Fn: Fn,
    //         Ft: Ft,
    //         Fm: Vec::new(),
    //         materials: Vec::new(),
    //         transform: transform,
    //         bbox: Aabb::new()
    //     };

    //     let rc_mesh = Rc::new(my_mesh);

    //     for i in 0..n_triangles{
    //         let triangle = Triangle{mesh : rc_mesh.clone(), face_idx: i};
    //     }
    // }

    pub fn empty(&self) -> bool {
        self.Fv.is_empty() | self.vs.is_empty()
    }
}

pub struct Triangle {
    pub mesh: Rc<Mesh>,
    pub face_idx: usize,
}

impl Triangle {
    /// convenience function to access the i-th vertex (i must be 0, 1, or 2)
    pub fn vertex(&self, i: usize) -> Vec3{
        self.mesh.vs[self.mesh.Fv[self.face_idx][i]]
    }
}

impl Surface for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        // vertices
        let iv0 = self.mesh.Fv[self.face_idx].x;
        let iv1 = self.mesh.Fv[self.face_idx].y;
        let iv2 = self.mesh.Fv[self.face_idx].z;

        let p0 = self.mesh.vs[iv0];
        let p1 = self.mesh.vs[iv1];
        let p2 = self.mesh.vs[iv2];

        // shading normals
        // const Vec3f *n0 = nullptr, *n1 = nullptr, *n2 = nullptr;
        let mut n0: Option<Vector3<f32>> = None;
        let mut n1: Option<Vector3<f32>> = None;
        let mut n2: Option<Vector3<f32>> = None;
        if self.mesh.Fn.len() > self.face_idx
        {
            let in0 = self.mesh.Fn[self.face_idx].x;
            let in1 = self.mesh.Fn[self.face_idx].y;
            let in2 = self.mesh.Fn[self.face_idx].z;
            if in0 >= 0 && in1 >= 0 && in2 >= 0
            {
                // spdlog::info("shading normals");
                n0.replace(self.mesh.ns[in0]);
                n1.replace(self.mesh.ns[in1]);
                n2.replace(self.mesh.ns[in2]);
            }
        }
        // texture coordinates
        let mut t0: Option<Vector2<f32>> = None;
        let mut t1: Option<Vector2<f32>> = None;
        let mut t2: Option<Vector2<f32>> = None;
        if self.mesh.Ft.len() > self.face_idx
        {
            let it0 = self.mesh.Ft[self.face_idx].x;
            let it1 = self.mesh.Ft[self.face_idx].y;
            let it2 = self.mesh.Ft[self.face_idx].z;
            if it0 >= 0 && it1 >= 0 && it2 >= 0
            {
                t0.replace(self.mesh.uvs[it0]);
                t1.replace(self.mesh.uvs[it1]);
                t2.replace(self.mesh.uvs[it2]);
            }
        }
        let material = self.mesh.materials.clone();
        return single_triangle_intersect(ray, &p0, &p1, &p2, &n0, &n1, &n2, &t0, &t1, &t2, material);

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
        for i in 0..3{
            if diag[i] < 1e-4{
                aabb.min[i] -= 5e-5;
                aabb.max[i] += 5e-5;
            }
        }
        aabb
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
    use serde_json::json;

    extern crate approx;

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

            approx::assert_abs_diff_eq!(correct_t, hit.t, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_p, hit.p, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_gn, hit.gn, epsilon = 1e-5);
            approx::assert_abs_diff_eq!(correct_sn, hit.sn, epsilon = 1e-5);
        } else {
            assert!(false, "did not hit")
        }
    }
}
