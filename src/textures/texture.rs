extern crate nalgebra_glm as glm;


use enum_dispatch::enum_dispatch;
use glm::Vec3;

use crate::image2d::Image2d;
use crate::surfaces::surface::HitInfo;
use crate::transform::Transform;
use crate::textures::perlin;

#[enum_dispatch]
pub trait Texture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3>;
}

#[enum_dispatch(Texture)]
#[derive(Debug, PartialEq, Clone)]
pub enum TextureType {
    ConstantTexture,
    CheckerTexture,
    ImageTexture,
    MarbleTexture
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstantTexture {
    pub color: Vec3,
}

impl Texture for ConstantTexture {
    fn value(&self, _hit: &HitInfo) -> Option<Vec3> {
        Some(self.color)
    }
}

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
        let sines = (p.x / self.scale).sin() *  (p.y / self.scale).sin() * (p.z / self.scale).sin();
        if sines < 0.0 {
            self.odd_texture.value(hit)
        } else {
            self.even_texture.value(hit)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ImageTexture {
    image: Image2d,
}

impl Texture for ImageTexture {
    fn value(&self, _hit: &HitInfo) -> Option<Vec3> {
        None
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MarbleTexture{
    pub base: Box<TextureType>,
    pub veins: Box<TextureType>,
    pub scale: f32,
    pub transform: Transform
}

impl Texture for MarbleTexture {
    fn value(&self, hit: &HitInfo) -> Option<Vec3> {
        // return lerp(m_veins->value(hit), m_base->value(hit),  0.5f * (1.f + std::sin(m_scale * hit.p.z + 10 *perlin.turbulant_noise(hit, 1.0))));
        let t = 0.5 * (1.0 + (self.scale * hit.p.z + 10.0 * perlin::turb(hit.p, self.scale, 7)).sin());
        let v = glm::lerp(&self.veins.value(hit).unwrap(), &self.base.value(hit).unwrap(), t);
        
        Some(v)
        // None

    }
}





