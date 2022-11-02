extern crate nalgebra_glm as glm;
use glm::Vec3;

use crate::surfaces::surface::HitInfo;
use crate::textures::perlin;
use crate::textures::texture::{create_texture, Texture, TextureType};
use crate::transform::{read_transform, Transform};
use crate::utils::read;
use serde_json::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct MarbleTexture {
    base: Box<TextureType>,
    veins: Box<TextureType>,
    scale: f32,
    transform: Transform,
}

impl Texture for MarbleTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        let t = 0.5
            * (1.0
                + f32::sin(self.scale * hit.p.z + 10.0 * perlin::turbulant_noise(hit.p, 1.0, 7)));
        let v = glm::lerp(
            &self.veins.value(hit).unwrap(),
            &self.base.value(hit).unwrap(),
            t,
        );

        Some(v)
    }
}

impl MarbleTexture {
    pub fn new(v: &Value) -> MarbleTexture {
        let veins = Box::new(create_texture(&v, "veins"));
        let base = Box::new(create_texture(&v, "base"));
        let scale = read::<f32>(&v, "scale");
        let transform = read_transform(&v);
        MarbleTexture {
            base: base,
            veins: veins,
            scale: scale,
            transform: transform,
        }
    }
}
