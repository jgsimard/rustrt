use nalgebra_glm::Vec3;
use serde_json::Value;

use crate::core::image2d::Image2d;
use crate::core::utils::read;
use crate::surfaces::HitInfo;
use crate::textures::Texture;

#[derive(Debug, PartialEq, Clone)]
pub struct ImageTexture {
    image: Image2d,
}

impl Texture for ImageTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let x = (self.image.size_x as f32) * hit.uv.x;
        let y = (self.image.size_y as f32) * (1.0 - hit.uv.y);
        let v = self.image[(x as usize, y as usize)];
        Some(v)
    }
}

impl ImageTexture {
    pub fn new(v: &Value) -> ImageTexture {
        let filename: String = read(v, "filename");
        let image = Image2d::load(filename);

        ImageTexture { image }
    }
}
