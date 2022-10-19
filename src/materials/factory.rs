use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::MaterialType;
use crate::materials::metal::Metal;
use crate::utils::{read_v_or_f, Factory};

extern crate nalgebra_glm as glm;
use glm::Vec3;
use serde_json::from_value;
use std::collections::HashMap;
use std::rc::Rc;

use serde_json::Value;

pub struct MaterialFactory {
    pub materials: HashMap<String, Rc<MaterialType>>,
}

impl MaterialFactory {
    pub fn new() -> MaterialFactory {
        MaterialFactory {
            materials: HashMap::new(),
        }
    }
}

impl Factory<Rc<MaterialType>> for MaterialFactory {
    fn make(&mut self, v: &Value) -> Option<Vec<Rc<MaterialType>>> {
        let m = v.as_object().unwrap();
        let name = m
            .get("name")
            .expect("Feature doesnt have name")
            .to_string()
            .trim_matches('"')
            .to_string();
        let material = create_material((*v).clone());
        self.materials.insert(name, material.clone());
        Some(vec![material])
    }
}

pub fn create_material(material_json: Value) -> Rc<MaterialType> {
    let type_material = material_json
        .get("type")
        .expect("material should have a type")
        .as_str()
        .expect("material type should be a string");

    match type_material {
        "lambertian" => {
            let albedo = read_v_or_f(&material_json, "albedo", Vec3::zeros());
            Rc::new(MaterialType::from(Lambertian { albedo }))
        }
        "metal" => {
            let albedo = read_v_or_f(&material_json, "albedo", Vec3::zeros());
            let roughness = material_json
                .get("roughness")
                .map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());
            Rc::new(MaterialType::from(Metal { albedo, roughness }))
        }
        "dielectric" => {
            let ior = material_json
                .get("ior")
                .map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());
            Rc::new(MaterialType::from(Dielectric { ior }))
        }
        "diffuse_light" => {
            let emit = read_v_or_f(&material_json, "emit", Vec3::new(1.0, 1.0, 1.0));
            Rc::new(MaterialType::from(DiffuseLight { emit }))
        }
        _ => unimplemented!("The material type '{}' ", type_material),
    }
}
