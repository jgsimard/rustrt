extern crate nalgebra_glm as glm;
use crate::utils::read;
use glm::Vec3;
use serde_json::{from_value, Value};

use crate::surfaces::surface::HitInfo;
use crate::textures::texture::Texture;

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantTexture {
    color: Vec3,
}

impl Texture for ConstantTexture {
    fn value(&self, _hit: &HitInfo) -> Option<Vec3> {
        Some(self.color)
    }
}

impl ConstantTexture {
    pub fn new(v: &Value) -> ConstantTexture {
        let color = if v.is_number() {
            let thing_number = from_value::<f32>(v.clone()).unwrap();
            Vec3::new(thing_number, thing_number, thing_number)
        } else if v.is_array() {
            from_value::<Vec3>(v.clone()).unwrap()
        } else if v.is_object() {
            read::<Vec3>(v, "color")
        } else {
            panic!("unable to read texture {:?}", v);
        };
        ConstantTexture { color }
    }
}

#[cfg(test)]
mod tests {

    use nalgebra_glm::Vec3;
    use serde_json::json;

    use crate::textures::constant::ConstantTexture;
    use crate::textures::texture::create_texture;

    use crate::textures::texture::TextureType;

    #[test]
    fn create_texture_number() {
        let v = json!({
            "albedo": 1.0
        });

        let texture = create_texture(&v, "albedo");
        let target_texture = TextureType::from(ConstantTexture {
            color: Vec3::new(1.0, 1.0, 1.0),
        });
        assert_eq!(target_texture, texture);
    }

    #[test]
    fn create_texture_vector() {
        let v = json!({
            "albedo": [1.0, 1.0, 1.0]
        });

        let texture = create_texture(&v, "albedo");
        let target_texture = TextureType::from(ConstantTexture {
            color: Vec3::new(1.0, 1.0, 1.0),
        });
        assert_eq!(target_texture, texture);
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
        let target_texture = TextureType::from(ConstantTexture {
            color: Vec3::new(0.73, 0.73, 0.73),
        });
        assert_eq!(target_texture, texture);
    }
}
