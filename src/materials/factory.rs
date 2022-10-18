use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::Material;
use crate::materials::metal::Metal;
use crate::utils::{read_v_or_f, Factory};

use nalgebra::Vector3;
use serde_json::from_value;
use std::collections::HashMap;
use std::rc::Rc;

use serde_json::Value;

pub struct MaterialFactory {
    pub materials: HashMap<String, Rc<dyn Material>>,
}

impl MaterialFactory {
    pub fn new() -> MaterialFactory {
        MaterialFactory {
            materials: HashMap::new(),
        }
    }
}

impl Factory<Rc<dyn Material>> for MaterialFactory {
    fn make(&mut self, v: &Value) -> Option<Vec<Rc<dyn Material>>> {
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

pub fn create_material(material_json: Value) -> Rc<dyn Material> {
    let type_material = material_json
        .get("type")
        .expect("material should have a type");
    if type_material == "lambertian" {
        let albedo = read_v_or_f(&material_json, "albedo", Vector3::zeros());
        Rc::new(Lambertian { albedo: albedo })
    } else if type_material == "metal" {
        let albedo = read_v_or_f(&material_json, "albedo", Vector3::zeros());
        let roughness = material_json
            .get("roughness")
            .map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());
        Rc::new(Metal {
            albedo: albedo,
            roughness: roughness,
        })
    } else if type_material == "dielectric" {
        let ior = material_json
            .get("ior")
            .map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());
        Rc::new(Dielectric { ior: ior })
    } else if type_material == "diffuse_light" {
        let emit = read_v_or_f(&material_json, "emit", Vector3::new(1.0, 1.0, 1.0));
        Rc::new(DiffuseLight { emit: emit })
    } else {
        panic!(
            "The material type '{}' is not yet implemented",
            type_material
        )
    }
}
