extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::surfaces::surface::HitInfo;
use crate::textures::texture::{Texture, TextureType};
use crate::transform::Transform;

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
