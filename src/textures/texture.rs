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

#[cfg(test)]
mod tests {

    use nalgebra_glm::Vec3;
    use serde_json::json;

    use crate::textures::texture::create_texture;
    use crate::textures::constant::ConstantTexture;

    use super::TextureType;

    #[test]
    #[should_panic]
    fn create_texture_panic() {
        let v = json!({
            "albedo": "XX"
        });
        create_texture(&v, "albedo");        
    }

    #[test]
    fn create_texture_number() {
        let v = json!({
            "albedo": 1.0
        });

        let texture = create_texture(&v, "albedo");        
        assert!(TextureType::ConstantTexture( ConstantTexture{ color: Vec3::new(1.0, 1.0, 1.0) }) == texture);
    }

    #[test]
    fn create_texture_vector() {
        let v = json!({
            "albedo": [1.0, 1.0, 1.0]
        });

        let texture = create_texture(&v, "albedo");        
        assert!(TextureType::ConstantTexture( ConstantTexture{ color: Vec3::new(1.0, 1.0, 1.0) }) == texture);
    }

    #[test]
    fn create_texture_constant() {
        let v = json!({
            "albedo": {
                "type": "constant",
                "color": [0.73, 0.73, 0.73]
            }
        });

        let texture = create_texture(&v, "albedo");        
        assert!(TextureType::ConstantTexture( ConstantTexture{ color: Vec3::new(0.73, 0.73, 0.73) }) == texture);
    }

    #[test]
    fn create_texture_checker() {
        let v = json!({
            "albedo": {
                "type": "checker",
                "even": [
                    0.2, 0.3, 0.1
                ],
                "odd": [
                    0.9, 0.9, 0.9
                ],
                "scale": 0.1
            } 
        });

        let texture = create_texture(&v, "albedo");    
        match texture {
            TextureType::CheckerTexture{ .. } => {}
            _ => {panic!("Did not work")}
        }    
    }

    #[test]
    fn create_texture_marble1() {
        let v = json!({
            "albedo": {
                "type": "marble",
                "scale": 2,
                "veins": 0,
                "base": 0.9
            } 
        });

        let texture = create_texture(&v, "albedo");    
        match texture {
            TextureType::MarbleTexture{ .. } => {}
            _ => {panic!("Did not work")}
        }    
    }

    #[test]
    fn create_texture_marble2() {
        let v = json!({
            "albedo": {
                "type": "marble",
                "scale": 3,
                "veins": [
                    0.08, 0.1, 0.08
                ],
                "base": [0.38, 0.4, 0.38]
            } 
        });

        let texture = create_texture(&v, "albedo");    
        match texture {
            TextureType::MarbleTexture{ .. } => {}
            _ => {panic!("Did not work")}
        }    
    }

    #[test]
    fn create_texture_image() {
        let v = json!({
            "albedo": {
                "type": "image",
                "filename": "assets/earth.jpg"
            } 
        });

        let texture = create_texture(&v, "albedo");    
        match texture {
            TextureType::ImageTexture{ .. } => {}
            _ => {panic!("Did not work")}
        }    
    }
}
