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
            read::<Vec3>(&v, "color")
        } else {
            panic!("unable to read texture {:?}", v);
        };
        ConstantTexture { color }
    }
}
