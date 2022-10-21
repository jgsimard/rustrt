use crate::aabb::Aabb;
// use crate::materials::factory::create_material;
use crate::materials::factory::MaterialFactory;
use crate::materials::material::MaterialType;
use crate::surfaces::quad::Quad;
use crate::surfaces::sphere::Sphere;
use crate::surfaces::surface::SurfaceType;
use crate::surfaces::triangle::{Mesh, Triangle};
use crate::transform::read_transform;
use crate::utils::{read, read_or, Factory};
use nalgebra::Vector3;
use serde_json::{Map, Value};
use std::rc::Rc;
extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use tobj;

pub struct SurfaceFactory {
    pub material_factory: MaterialFactory,
}

impl Factory<SurfaceType> for SurfaceFactory {
    fn make(&mut self, v: &Value) -> Option<Vec<SurfaceType>> {
        let m = v.as_object().unwrap();
        let surface_type = m.get("type").unwrap().as_str().unwrap();

        match surface_type {
            "sphere" => {
                let radius = read_or(v, "radius", 1.0);
                let transform = read_transform(v);
                let material = self.get_material(m);

                return Some(vec![SurfaceType::from(Sphere {
                    radius,
                    transform,
                    material,
                })]);
            }
            "quad" => {
                let size = if m.get("size").unwrap().is_number() {
                    let s = read(v, "size");
                    Vec2::new(s, s)
                } else {
                    read::<Vec2>(v, "size")
                };
                let size = size / 2.0;

                let transform = read_transform(v);
                let material = self.get_material(m);

                return Some(vec![SurfaceType::from(Quad {
                    size,
                    transform,
                    material,
                })]);
            }
            "mesh" => {
                let transform = read_transform(v);
                let filename: String = read(v, "filename");

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
                return Some(
                    (0..n_triangles)
                        .into_iter()
                        .map(|i| {
                            SurfaceType::from(Triangle {
                                mesh: rc_mesh.clone(),
                                face_idx: i,
                            })
                        })
                        .collect(),
                );
            }
            _ => unimplemented!("surface type {} not supported", surface_type),
        }
    }
}

impl SurfaceFactory {
    fn get_material(&self, m: &Map<String, Value>) -> Rc<MaterialType> {
        let material = if let Some(mat) = m.get("material") {
            if mat.is_string() {
                (*self
                    .material_factory
                    .materials
                    .get(mat.as_str().unwrap())
                    .unwrap())
                .clone()
            } else {
                self.material_factory.create_material((*mat).clone())
            }
        } else {
            panic!("Invalid material");
        };
        return material;
    }
}
