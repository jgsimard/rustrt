extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::surfaces::surface::HitInfo;
use crate::textures::perlin;
use crate::textures::texture::{Texture, TextureType};
use crate::transform::Transform;

#[derive(Debug, PartialEq, Clone)]
pub struct MarbleTexture {
    pub base: Box<TextureType>,
    pub veins: Box<TextureType>,
    pub scale: f32,
    pub transform: Transform,
}

impl Texture for MarbleTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let t =
            0.5 * (1.0 + (self.scale * hit.p.z + 10.0 * perlin::turb(hit.p, self.scale, 7)).sin());
        let v = glm::lerp(
            &self.veins.value(hit).unwrap(),
            &self.base.value(hit).unwrap(),
            t,
        );

        Some(v)
        // None
    }
}
