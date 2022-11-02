extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::surfaces::surface::HitInfo;
use crate::textures::texture::{create_texture, Texture, TextureType};
use crate::transform::{read_transform, Transform};
use crate::utils::read;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct CheckerTexture {
    pub odd_texture: Box<TextureType>,
    pub even_texture: Box<TextureType>,
    pub scale: f32,
    pub transform: Transform,
}

impl Texture for CheckerTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let p = self.transform.point(&hit.p);
        // let sines = (p.x * self.scale).sin() *  (p.y * self.scale).sin() * (p.z * self.scale).sin();
        let sines = (p.x / self.scale).sin() * (p.y / self.scale).sin() * (p.z / self.scale).sin();
        if sines < 0.0 {
            self.odd_texture.value(hit)
        } else {
            self.even_texture.value(hit)
        }
    }
}

impl CheckerTexture {
    pub fn new(v: &Value) -> CheckerTexture {
        let even = Box::new(create_texture(&v, "even"));
        let odd = Box::new(create_texture(&v, "odd"));
        let scale = read::<f32>(&v, "scale");
        let transform = read_transform(&v);

        return CheckerTexture {
            odd_texture: odd,
            even_texture: even,
            scale: scale,
            transform: transform,
        };
    }
}
