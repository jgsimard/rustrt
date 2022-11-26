use image::{io::Reader as ImageReader, GenericImageView, Pixel};
use nalgebra_glm::{clamp, comp_max, comp_min, Vec3};
use std::ops::{Index, IndexMut};
use std::path::Path;

use image::Rgb;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array2d<T> {
    pub data: Vec<T>,
    pub size_x: usize,
    pub size_y: usize,
}

impl<T: std::clone::Clone + std::default::Default> Array2d<T> {
    fn index_1d(&self, x: usize, y: usize) -> usize {
        y * self.size_x + x
    }

    pub fn size(&self) -> usize {
        self.size_x * self.size_y
    }

    pub fn new(size_x: usize, size_y: usize) -> Array2d<T> {
        Array2d::<T> {
            data: vec![T::default(); size_x * size_y],
            size_x,
            size_y,
        }
    }
}

impl<T: std::clone::Clone + std::default::Default> Index<(usize, usize)> for Array2d<T> {
    type Output = T;

    fn index(&self, pos: (usize, usize)) -> &Self::Output {
        let index = self.index_1d(pos.0, pos.1);
        &self.data[index]
    }
}

impl<T: std::clone::Clone + std::default::Default> IndexMut<(usize, usize)> for Array2d<T> {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut Self::Output {
        let index = self.index_1d(pos.0, pos.1);
        &mut self.data[index]
    }
}
pub type Image2d = Array2d<Vec3>;

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

    result
}

impl Image2d {
    pub fn save(&self, path: &Path) {
        // let mut img_buffer = image::Rgb32FImage::new(self.size_x as u32, self.size_y as u32);
        let mut img_buffer = image::RgbImage::new(self.size_x as u32, self.size_y as u32);
        let mut max = f32::MIN;
        let mut min = f32::MAX;
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                let v = self[(x, y)];
                let max_ = comp_max(&v);
                let min_ = comp_min(&v);
                if max_ > max {
                    max = max_;
                }
                if min_ < min {
                    min = min_;
                }
                let v = self[(x, y)];
                // let v = glm::clamp(&v, 0.0, 1.0);
                // let v = glm::sqrt(&v);
                // img_buffer.put_pixel(x as u32, y as u32, Rgb([v.x, v.y, v.z]));
                let v = to_srgb(&v);
                let v = clamp(&v, 0.0, 1.0) * 255.0;
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
                let r = f32::from(pixel[0]) / 255.0;
                let g = f32::from(pixel[1]) / 255.0;
                let b = f32::from(pixel[2]) / 255.0;
                image2d[(x, y)] = Vec3::new(r, g, b);
            }
        }
        image2d
    }
}
