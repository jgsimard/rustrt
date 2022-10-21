use crate::image2d::Image2d;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::fresnel_blend::FresnelBlend;
use crate::materials::lambertian::Lambertian;
use crate::materials::material::MaterialType;
use crate::materials::metal::Metal;
use crate::materials::phong::Phong;
use crate::textures::texture::{
    CheckerTexture, ConstantTexture, ImageTexture, MarbleTexture, TextureType,
};
use crate::transform::read_transform;
use crate::utils::{read, read_v_or_f, Factory, read_or};

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

    pub fn create_material(&self, material_json: Value) -> Rc<MaterialType> {
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
                let ior = create_texture(&material_json, "ior");
                Rc::new(MaterialType::from(Dielectric { ior }))
            }
            "diffuse_light" => {
                let emit = read_v_or_f(&material_json, "emit");
                Rc::new(MaterialType::from(DiffuseLight { emit }))
            }
            "fresnel_blend" => {
                let ior = create_texture(&material_json, "ior");
                let v = material_json.get("refr").unwrap().clone();
                let refracted = if v.is_string() {
                    let refracted_name: String = from_value(v).unwrap();
                    (*self
                        .materials
                        .get(&refracted_name)
                        .expect("doesnt contain refr"))
                    .clone()
                } else if v.is_object() {
                    self.create_material(v)
                } else {
                    panic!("NOOOOOO refr : {}", v);
                };

                let v = material_json.get("refl").expect("no refl").clone();
                let reflected = if v.is_string() {
                    let reflected_name: String = from_value(v).unwrap();
                    (*self
                        .materials
                        .get(&reflected_name)
                        .expect("doesnt contain refl"))
                    .clone()
                } else if v.is_object() {
                    self.create_material(v)
                } else {
                    panic!("NOOOOOO refl : {}", v);
                };

                Rc::new(MaterialType::from(FresnelBlend {
                    ior: ior,
                    refracted: refracted,
                    reflected: reflected,
                }))
            }
            "phong" => {
                let albedo = create_texture(&material_json, "albedo");
                let exponent = read_or(&material_json, "exponent", 1.0);
                Rc::new(MaterialType::from(Phong { albedo, exponent }))
            }
            _ => unimplemented!("The material type '{}' ", type_material),
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
        let material = self.create_material((*v).clone());
        self.materials.insert(name, material.clone());
        Some(vec![material])
    }
}

pub fn create_texture(j: &Value, thing_name: &str) -> TextureType {
    let v = j.get(thing_name).unwrap().clone();
    let texture = if v.is_number() {
        let thing_number: f32 = from_value(v).unwrap();
        let color = Vec3::new(thing_number, thing_number, thing_number);
        TextureType::from(ConstantTexture { color: color })
    } else if v.is_array() {
        let color = read::<Vec3>(j, thing_name);
        TextureType::from(ConstantTexture { color: color })
    } else if v.is_object() {
        let texture_type = v
            .get("type")
            .expect("no texture type")
            .as_str()
            .expect("lolz");

        match texture_type {
            "constant" => {
                let color = read::<Vec3>(&v, "color");
                TextureType::from(ConstantTexture { color: color })
            }
            "checker" => {
                let even = Box::new(create_texture(&v, "even"));
                let odd = Box::new(create_texture(&v, "odd"));
                let scale = read::<f32>(&v, "scale");
                let transform = read_transform(&v);

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
                let scale = read::<f32>(&v, "scale");
                let transform = read_transform(&v);
                TextureType::from(MarbleTexture {
                    base: base,
                    veins: veins,
                    scale: scale,
                    transform: transform,
                })
            }
            "image" => {
                let filename: String = read(&v, "filename");
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
