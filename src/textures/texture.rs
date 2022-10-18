// extern crate nalgebra_glm as glm;
// use glm::Vec3;
// use enum_dispatch::enum_dispatch;

// use crate::surfaces::surface::HitInfo;
// use crate::image2d::Image2d;

// #[enum_dispatch]
// pub trait Texture{
//     fn value(&self, hit: &HitInfo) -> Option<Vec3>;
// }

// #[enum_dispatch(Texture)]
// enum TextureType {
//     ConstantTexture,
//     CheckerTexture,
//     // ImageTexture
// }

// pub struct ConstantTexture{
//     pub color: Vec3
// }

// pub struct CheckerTexture{
//     odd_texture: Box<TextureType>,
//     even_texture: Box<TextureType>,
//     scale: f32
// }

// // pub struct ImageTexture{
// //     image: Image2d
// // }

// // pub struct MarbleTexture{
// //     base: Box<dyn Texture>,
// //     veins: Box<dyn Texture>,
// //     scale: f32,
// //     perlin: Perlin
// // }

// // pub struct Perlin{
// //     point_count: i32,
// //     random_vectors: Vec<Vec3>,
// //     perm_x: Vec<i32>,
// //     perm_y: Vec<i32>,
// //     perm_z: Vec<i32>
// // }

// impl Texture for ConstantTexture {
//     fn value(&self, _hit: &HitInfo) -> Option<Vec3> {
//         Some(self.color)
//     }
// }

// impl Texture for CheckerTexture {
//     fn value(&self, hit: &HitInfo) -> Option<Vec3> {
//         None
//     }
// }
