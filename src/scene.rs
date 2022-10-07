use nalgebra::{Vector2, Vector3};
use serde_json::Value;
use std::rc::Rc;
use std::fmt::Write;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};

use crate::camera::PinholeCamera;
use crate::ray::Ray;
use crate::surface::{Surface, HitInfo, SurfaceGroup};
use crate::image2d::Image2d;


pub struct Scene{
    pub surfaces: SurfaceGroup,
    // pub emitters: SurfaceGroup,
    // pub integrator: Rc<dyn Integrator>,
    // pub sampler: Rc<dyn Sampler>,
    pub camera: PinholeCamera,
    pub background: Vector3<f32>,
    pub num_samples: u32,
    pub max_depth: i32
}


impl Scene {
    pub fn new(scene_json: Value) -> Scene{
        // Scene{surfaces: Vec::new()}
    }
    pub fn add_child(&mut self, surface: Rc<dyn Surface>)
    {
        self.surfaces.add_child(surface);
    }

    pub fn add_to_parent(&self)
    { 
        
    }

    fn recursive_color(&self, ray: &Ray, depth: i32) -> Vector3<f32>
    {
        const BLACK: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        if let Some(hit) = self.intersect(ray)
        {
            if depth < self.max_depth
            {
                if let Some((attenuation, scattered)) = hit.mat.scatter(ray, &hit){
                    return attenuation.component_mul(&self.recursive_color(&scattered, depth + 1));
                }
                return BLACK;
            }
            else
            {
                return BLACK;
            }
        }
        else
        {
            return  self.background;
        }
    }

    pub fn raytrace(&self) -> Image2d
    {
        let mut image = Image2d::new(self.camera.size.x as usize, self.camera.size.y as usize);
        let sample_count = self.num_samples;

        {
            let progress_bar = ProgressBar::new(image.size() as u64);
            progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));
            println!("Rendering ...");
            // Generate a ray for each pixel in the ray image
            for y in 0..image.size_y{
                for x in 0..image.size_x{
                    let mut color = Vector3::new(0.0, 0.0, 0.0);
                    for _ in 0..sample_count{
                        // rays_traced.inc(1);
                        let ray = self.camera.generate_ray(&Vector2::new((x as f32) + 0.5, (y as f32) + 0.5));
                        color += self.recursive_color(&ray, 0) / (sample_count as f32);
                    }
                    image[(x, y)] = color;
                    progress_bar.inc(1);
                }
            }
            println!("Rendering time : {:?}", progress_bar.elapsed());
        } // progress reporter goes out of scope here
        return image;
    }
}

impl Surface for Scene {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        self.surfaces.intersect(ray)
    }    
}

