extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::surfaces::surface::HitInfo;
use crate::textures::texture::Texture;

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl Texture for ConstantTexture {
    fn value(&self, _hit: &HitInfo) -> Option<Vec3> {
        Some(self.color)
    }
}
