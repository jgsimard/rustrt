use crate::materials::factory::create_material;
use crate::materials::factory::read_vector2_f32;
use crate::materials::factory::MaterialFactory;
use crate::surfaces::quad::Quad;
use crate::surfaces::sphere::Sphere;
use crate::surfaces::surface::Surface;
use crate::transform::parse_transform;
use crate::utils::Factory;
use nalgebra::Vector2;
use serde_json::{from_value, Value};
use std::rc::Rc;

pub struct SurfaceFactory {
    pub material_factory: MaterialFactory,
}

impl Factory<Rc<dyn Surface>> for SurfaceFactory {
    fn make(&mut self, v: &Value) -> Option<Rc<dyn Surface>> {
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

            return Some(Rc::new(Sphere {
                radius,
                transform,
                material,
            }));
        } else if t == "quad" {
            let size = if m.get("size").unwrap().is_number() {
                let s = m.get("size").unwrap().as_f64().unwrap() as f32;
                Vector2::new(s, s)
            } else {
                read_vector2_f32(v, "size", Vector2::new(1.0, 1.0))
            };
            let size = size / 2.0;

            // let transform = parse_transform(&v["transform"]);
            let transform = if m.contains_key("transform") {
                parse_transform(&v["transform"])
            } else {
                Default::default()
            };
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

            return Some(Rc::new(Quad {
                size,
                transform,
                material,
            }));
        }
        None
    }
}
