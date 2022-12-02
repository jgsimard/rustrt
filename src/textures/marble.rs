use nalgebra_glm::{lerp, Vec3};
use serde_json::Value;

use crate::core::transform::{read_transform, Transform};
use crate::core::utils::read;
use crate::surfaces::HitInfo;
use crate::textures::perlin;
use crate::textures::{create_texture, Texture, TextureType};

#[derive(Debug, PartialEq, Clone)]
pub struct MarbleTexture {
    base: Box<TextureType>,
    veins: Box<TextureType>,
    scale: f32,
    transform: Transform,
}

impl Texture for MarbleTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let perlin_noise = perlin::turbulant_noise(hit.p, 1.0, 7);
        let t = 0.5 * (1.0 + f32::sin(self.scale * hit.p.z + 10.0 * perlin_noise));
        let v = lerp(&self.veins.value(hit)?, &self.base.value(hit)?, t);
        Some(v)
    }
}

impl MarbleTexture {
    pub fn new(v: &Value) -> MarbleTexture {
        let veins = Box::new(create_texture(v, "veins"));
        let base = Box::new(create_texture(v, "base"));
        let scale = read(v, "scale");
        let transform = read_transform(v);
        MarbleTexture {
            base,
            veins,
            scale,
            transform,
        }
    }
}
