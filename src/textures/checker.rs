use nalgebra_glm::Vec3;
use serde_json::Value;

use crate::core::transform::Transform;
use crate::core::utils::read;
use crate::surfaces::HitInfo;
use crate::textures::{create_texture, Texture, TextureType};

#[derive(Debug, PartialEq, Clone)]
pub struct CheckerTexture {
    odd_texture: Box<TextureType>,
    even_texture: Box<TextureType>,
    scale: f32,
    transform: Transform,
}

impl Texture for CheckerTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let p = self.transform.point(&hit.p);
        let sines = (p.x * self.scale).sin() * (p.y * self.scale).sin() * (p.z * self.scale).sin();
        // let sines = (p.x / self.scale).sin() * (p.y / self.scale).sin() * (p.z / self.scale).sin();
        if sines < 0.0 {
            self.odd_texture.value(hit)
        } else {
            self.even_texture.value(hit)
        }
    }
}

impl CheckerTexture {
    pub fn new(v: &Value) -> CheckerTexture {
        let even_texture = Box::new(create_texture(v, "even"));
        let odd_texture = Box::new(create_texture(v, "odd"));
        let scale = read::<f32>(v, "scale");
        let transform = Transform::read(v);

        CheckerTexture {
            odd_texture,
            even_texture,
            scale,
            transform,
        }
    }
}
