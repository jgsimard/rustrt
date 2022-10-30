extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::image2d::Image2d;
use crate::surfaces::surface::HitInfo;
use crate::textures::texture::Texture;

#[derive(Debug, PartialEq, Clone)]
pub struct ImageTexture {
    pub image: Image2d,
}

impl Texture for ImageTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let x = (self.image.size_x as f32) * hit.uv.x;
        let y = (self.image.size_y as f32) * (1.0 - hit.uv.y);
        let v = self.image[(x as usize, y as usize)];
        Some(v)
    }
}
