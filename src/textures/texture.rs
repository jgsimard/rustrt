use crate::surfaces::surface::HitInfo;
use enum_dispatch::enum_dispatch;
use nalgebra_glm::Vec3;
use serde_json::Value;

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
    let texture = if v.is_number() | v.is_array() {
        TextureType::from(ConstantTexture::new(&v))
    } else if v.is_object() {
        let texture_type = v
            .get("type")
            .expect("no texture type")
            .as_str()
            .expect("unable to get texture type");

        match texture_type {
            "constant" => TextureType::from(ConstantTexture::new(&v)),
            "checker" => TextureType::from(CheckerTexture::new(&v)),
            "marble" => TextureType::from(MarbleTexture::new(&v)),
            "image" => TextureType::from(ImageTexture::new(&v)),
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

    use serde_json::json;

    use crate::textures::texture::create_texture;

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
        assert!(matches!(texture, TextureType::CheckerTexture { .. }));
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
        assert!(matches!(texture, TextureType::MarbleTexture { .. }));
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
        assert!(matches!(texture, TextureType::MarbleTexture { .. }));
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

        assert!(matches!(texture, TextureType::ImageTexture { .. }));
    }
}
