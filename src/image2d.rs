extern crate nalgebra_glm as glm;

use image::Rgb32FImage;


use std::ops::{Index, IndexMut};
use nalgebra::Vector3;


use image::Rgb;

pub struct Image2d{
    data: Vec<Vector3<f32>>,
    pub size_x: usize,
    pub size_y: usize
}

impl Image2d {
    fn index_1d(&self, x: usize, y: usize) -> usize{ 
        y * self.size_x + x
    }

    pub fn new(size_x: usize, size_y: usize) -> Image2d{
        Image2d{
            data: vec![Vector3::zeros(); size_x * size_y],
            size_x: size_x,
            size_y: size_y
        }
    } 

    pub fn save(&self, path: String){ 
        let mut img_buffer = Rgb32FImage::new(self.size_x as u32, self.size_y as u32);
        
        for x in 0..self.size_x{
            for y in 0..self.size_y{
                let v = self[(x,y)];
                img_buffer.put_pixel(x as u32, y as u32, Rgb([v.x, v.y, v.z]));
            }
        }
        let img = image::DynamicImage::ImageRgb32F(img_buffer);
        let img = img.into_rgb8();
        img.save(path).unwrap();
    }
}

impl Index<(usize, usize)> for Image2d {
    type Output = Vector3<f32>;
    
    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        let index = self.index_1d(pos.0, pos.1); 
        &self.data[index]
    }
}

impl IndexMut<(usize, usize)> for Image2d {
    
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        let index = self.index_1d(pos.0, pos.1); 
        &mut self.data[index]
    }
}
