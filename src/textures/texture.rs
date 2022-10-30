use enum_dispatch::enum_dispatch;
use nalgebra_glm::Vec3;
use serde_json::{from_value, Value};
use crate::image2d::Image2d;
use crate::surfaces::surface::HitInfo;
use crate::transform::read_transform;
use crate::utils::read;



#[enum_dispatch]
pub trait Texture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3>;
}

use crate::textures::checker::CheckerTexture;
use crate::textures::constant::ConstantTexture;
use crate::textures::image::ImageTexture;
use crate::textures::marble::MarbleTexture;

#[enum_dispatch(Texture)]
#[derive(Debug, PartialEq, Clone)]
pub enum TextureType {
    ConstantTexture,
    CheckerTexture,
    ImageTexture,
    MarbleTexture,
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
