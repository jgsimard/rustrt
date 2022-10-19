extern crate nalgebra_glm as glm;
use glm::Vec3;
use image::{io::Reader as ImageReader, GenericImageView, Pixel};
use std::ops::{Index, IndexMut};

use image::Rgb;

#[derive(Debug, PartialEq, Clone)]
pub struct Image2d {
    data: Vec<Vec3>,
    pub size_x: usize,
    pub size_y: usize,
}

/// Convert from linear RGB to sRGB
fn to_srgb(c: &Vec3) -> Vec3 {
    let mut result = Vec3::new(0.0, 0.0, 0.0);

    for i in 0..3 {
        let value = c[i];
        if value <= 0.0031308 {
            result[i] = 12.92 * value;
        } else {
            result[i] = (1.0 + 0.055) * value.powf(1.0 / 2.4) - 0.055;
        }
    }

    return result;
}

impl Image2d {
    fn index_1d(&self, x: usize, y: usize) -> usize {
        y * self.size_x + x
    }

    pub fn size(&self) -> usize {
        self.size_x * self.size_y
    }

    pub fn new(size_x: usize, size_y: usize) -> Image2d {
        Image2d {
            data: vec![Vec3::zeros(); size_x * size_y],
            size_x: size_x,
            size_y: size_y,
        }
    }

    pub fn save(&self, path: String) {
        // let mut img_buffer = image::Rgb32FImage::new(self.size_x as u32, self.size_y as u32);
        let mut img_buffer = image::RgbImage::new(self.size_x as u32, self.size_y as u32);
        let mut max = f32::MIN;
        let mut min = f32::MAX;
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                let v = self[(x, y)];
                let max_ = glm::comp_max(&v);
                let min_ = glm::comp_min(&v);
                if max_ > max {
                    max = max_;
                }
                if min_ < min {
                    min = min_;
                }

                // let v = glm::sqrt(&self[(x, y)]);
                // img_buffer.put_pixel(x as u32, y as u32, Rgb([v.x, v.y, v.z]));
                let v = to_srgb(&self[(x, y)]);
                let v = glm::clamp(&v, 0.0, 1.0) * 255.0;
                img_buffer.put_pixel(x as u32, y as u32, Rgb([v.x as u8, v.y as u8, v.z as u8]));
            }
        }
        println!("raw image : min {}, max {}", min, max);
        // let img = image::DynamicImage::ImageRgb32F(img_buffer);
        // let img = img.into_rgb8();
        let img = image::DynamicImage::ImageRgb8(img_buffer);
        img.save(path).unwrap();
    }

    pub fn load(path: String) -> Image2d {
        let img = ImageReader::open(path).unwrap().decode().unwrap();
        let mut image2d = Image2d::new(img.width() as usize, img.height() as usize);

        for x in 0..image2d.size_x {
            for y in 0..image2d.size_y {
                let pixel = img.get_pixel(x as u32, y as u32);
                let pixel = pixel.to_rgb();
                let r = (pixel[0] as f32) / 255.0;
                let g = (pixel[1] as f32) / 255.0;
                let b = (pixel[2] as f32) / 255.0;
                image2d[(x, y)] = Vec3::new(r, g, b);
            }
        }
        return image2d;
    }
}

impl Index<(usize, usize)> for Image2d {
    type Output = Vec3;

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
// how to write unit tests !!!
// #[cfg(test)]
// mod tests{
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
