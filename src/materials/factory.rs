use crate::image2d::Image2d;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::MaterialType;
use crate::materials::metal::Metal;
use crate::textures::texture::{
    CheckerTexture, ConstantTexture, ImageTexture, MarbleTexture, TextureType,
};
use crate::transform::parse_transform;
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

use crate::utils::read_vector3_f32;

pub fn read_f32(v: &Value, name: &str) -> f32 {
    v.get(name)
        .map(|v: &Value| from_value::<f32>(v.clone()).expect("unable to read f32"))
        .unwrap()
}

pub fn read_vec3(v: &Value, name: &str) -> Vec3 {
    v.get(name)
        .map(|v: &Value| from_value::<Vec3>(v.clone()).expect("unable to read Vec3"))
        .unwrap()
}

pub fn create_texture(j: &Value, thing_name: &str) -> TextureType {
    let v = j.get(thing_name).unwrap().clone();
    let texture = if v.is_number() {
        let thing_number: f32 = from_value(v).unwrap();
        let color = Vec3::new(thing_number, thing_number, thing_number);
        TextureType::from(ConstantTexture { color: color })
    } else if v.is_array() {
        let color = read_vector3_f32(j, thing_name, Vec3::zeros());
        TextureType::from(ConstantTexture { color: color })
    } else if v.is_object() {
        let texture_type = v
            .get("type")
            .expect("no texture type")
            .as_str()
            .expect("lolz");

        match texture_type {
            "constant" => {
                let color = read_vec3(&v, "color");
                TextureType::from(ConstantTexture { color: color })
            }
            "checker" => {
                let even = Box::new(create_texture(&v, "even"));
                let odd = Box::new(create_texture(&v, "odd"));
                let scale = read_f32(&v, "scale");
                let transform = if v.as_object().unwrap().contains_key("transform") {
                    parse_transform(&v["transform"])
                } else {
                    Default::default()
                };
                TextureType::from(CheckerTexture {
                    odd_texture: odd,
                    even_texture: even,
                    scale: scale,
                    transform: transform,
                })
            }
            "marble" => {
                let veins = Box::new(create_texture(&v, "veins"));
                let base = Box::new(create_texture(&v, "base"));
                let scale = read_f32(&v, "scale");
                let transform = if v.as_object().unwrap().contains_key("transform") {
                    parse_transform(&v["transform"])
                } else {
                    Default::default()
                };
                TextureType::from(MarbleTexture {
                    base: base,
                    veins: veins,
                    scale: scale,
                    transform: transform,
                })
            }
            "image" => {
                let filename: String = from_value(v["filename"].clone()).expect("no filename");
                let image = Image2d::load(filename);

                TextureType::from(ImageTexture { image: image })
            }
            _ => {
                unimplemented!("Texture type {}", texture_type);
            }
        }
    } else {
        panic!("unable to read texture {:?}", v);
    };
    texture
}

pub fn create_material(material_json: Value) -> Rc<MaterialType> {
    let type_material = material_json
        .get("type")
        .expect("material should have a type")
        .as_str()
        .expect("material type should be a string");

    match type_material {
        "lambertian" => {
            let albedo = create_texture(&material_json, "albedo");
            Rc::new(MaterialType::from(Lambertian { albedo }))
        }
        "metal" => {
            let albedo = create_texture(&material_json, "albedo");
            let roughness = create_texture(&material_json, "roughness");
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
