use crate::aabb::Aabb;
use crate::materials::factory::create_material;
use crate::materials::factory::MaterialFactory;
use crate::materials::material::Material;
use crate::surfaces::quad::Quad;
use crate::surfaces::sphere::Sphere;
use crate::surfaces::surface::Surface;
use crate::surfaces::triangle::{Mesh, Triangle};
use crate::transform::parse_transform;
use crate::utils::read_vector2_f32;
use crate::utils::Factory;
use nalgebra::{Vector2, Vector3};
use serde_json::{from_value, Map, Value};
use std::rc::Rc;
extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use tobj;

pub struct SurfaceFactory {
    pub material_factory: MaterialFactory,
}

impl Factory<Rc<dyn Surface>> for SurfaceFactory {
    fn make(&mut self, v: &Value) -> Option<Vec<Rc<dyn Surface>>> {
        let m = v.as_object().unwrap();
        let t = m.get("type").unwrap();

        if t == "sphere" {
            let radius = if let Some(r) = m.get("radius") {
                from_value((*r).clone()).unwrap()
            } else {
                1.0
            };
            let transform = if m.contains_key("transform") {
                parse_transform(&v["transform"])
            } else {
                Default::default()
            };
            let material = self.get_material(m);

            return Some(vec![Rc::new(Sphere {
                radius,
                transform,
                material,
            })]);
        } else if t == "quad" {
            let size = if m.get("size").unwrap().is_number() {
                let s = m.get("size").unwrap().as_f64().unwrap() as f32;
                Vector2::new(s, s)
            } else {
                read_vector2_f32(v, "size", Vector2::new(1.0, 1.0))
            };
            let size = size / 2.0;

            let transform = if m.contains_key("transform") {
                parse_transform(&v["transform"])
            } else {
                Default::default()
            };
            let material = self.get_material(m);

            return Some(vec![Rc::new(Quad {
                size,
                transform,
                material,
            })]);
        } else if t == "mesh" {
            let transform = if v.as_object().unwrap().contains_key("transform") {
                parse_transform(&v["transform"])
            } else {
                Default::default()
            };

            let filename: String = from_value(v["filename"].clone()).expect("no filename");

            let obj = tobj::load_obj(filename, &tobj::OFFLINE_RENDERING_LOAD_OPTIONS);

            assert!(obj.is_ok());
            let (models, _) = obj.expect("Failed to load OBJ file");

            let mesh = &models[0].mesh;
            let vs: Vec<Vec3> = mesh
                .positions
                .chunks(3)
                .map(|p| transform.point(&Vec3::new(p[0], p[1], p[2])))
                .collect();

            let mut aabb = Aabb::new();
            for vertex in vs.iter() {
                aabb.enclose_point(&vertex);
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

            assert!(mesh.positions.len() % 3 == 0);

            let material = self.get_material(m);

            let n_triangles = vertex_indices.len();
            let my_mesh = Mesh {
                vertex_positions: vs,
                vertex_normals: ns,
                uvs: uvs,
                vertex_indices: vertex_indices,
                normal_indices: normal_indices,
                texture_indices: texture_indices,
                material_indices: Vec::new(),
                materials: material,
                transform: transform,
                bbox: aabb,
            };

            let rc_mesh = Rc::new(my_mesh);
            // return Some(vec![rc_mesh]);
            let mut triangles: Vec<Rc<dyn Surface>> = Vec::new();
            for i in 0..n_triangles {
                triangles.push(Rc::new(Triangle {
                    mesh: rc_mesh.clone(),
                    face_idx: i,
                }));
            }
            return Some(triangles);

            // let triangles : Vec<Rc<dyn Surface>> = (0..n_triangles).into_iter().map(|i| Rc::new(Triangle{mesh : rc_mesh.clone(), face_idx: i})).collect();
            // return Some(triangles);
            // return None

            // return Some(Rc::new(Sphere::new(1.0, material)));
        }
        None
    }
}

impl SurfaceFactory {
    fn get_material(&self, m: &Map<String, Value>) -> Rc<dyn Material> {
        let material = if let Some(mat) = m.get("material") {
            if mat.is_string() {
                (*self
                    .material_factory
                    .materials
                    .get(mat.as_str().unwrap())
                    .unwrap())
                .clone()
            } else {
                create_material((*mat).clone())
            }
        } else {
            panic!("Invalid material");
        };
        return material;
    }
}
