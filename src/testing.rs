extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use rand::Rng;
use serde_json::Value;
use std::fs;

use crate::image2d::{Array2d, Image2d};
use crate::materials::factory::MaterialFactory;
use crate::materials::material::{Material, MaterialType};
use crate::surfaces::surface::{HitInfo, SurfaceType, Surface, EmitterRecord};
use crate::utils::{
    direction_to_spherical_coordinates, inferno, read, read_or, spherical_coordinates_to_direction,
    FRAC_1_TWOPI, Factory
};
use crate::surfaces::surface::SurfaceGroupType;
use crate::surfaces::accelerators::LinearSurfaceGroup;
use crate::surfaces::factory::SurfaceFactory;

use std::f32::consts::FRAC_1_PI;
use std::f32::consts::PI;
use std::rc::Rc;

pub trait SampleTest {
    fn sample(&self, params: &mut SampleTestParameters, rv: &Vec2) -> Option<Vec3>;
    fn pdf(&self, params: &mut SampleTestParameters, dir: &Vec3) -> f32;
}

pub struct MaterialTest {
    material: Rc<MaterialType>,
    // normal: Vec3,
    incoming: Vec3,
    hit: HitInfo
}

pub struct SampleTestParameters {
    any_specular: bool,
    any_below_hemisphere: bool,

    name: String,
    image_width: usize,
    image_height: usize,
    num_samples: usize,
}


impl MaterialTest {
    // it is used in tests, but it gives me a warning : FIXME !
    #[allow(unused)]
    pub fn new(v: Value) -> (MaterialTest, SampleTestParameters) {
        let mf = MaterialFactory::new();
        let material = mf.create_material(v["material"].clone());
        let normal = glm::normalize(&read(&v, "normal"));
        let incoming = glm::normalize(&read_or(&v, "incoming", Vec3::new(0.25, 0.0, -1.0)));
        let hit = HitInfo {
            t: 1.0,
            p: Vec3::zeros(),
            gn: normal,
            sn: normal,
            uv: Vec2::new(0.5, 0.5),
            mat: material.clone(),
        };
        let name = read(&v, "name");
        let image_width = read_or(&v, "image_width", 512);
        let image_height = read_or(&v, "image_height", 256);
        let num_samples = read_or(&v, "num_samples", 50) * image_width * image_height;

        let test = MaterialTest {
            material: material.clone(),
            // normal: normal,
            incoming: incoming,
            hit: hit
        };
        let parameters = SampleTestParameters { 
            any_specular: false, 
            any_below_hemisphere: false, 
            name: name, 
            image_width: image_width, 
            image_height: image_height, 
            num_samples: num_samples 
        };
        (test, parameters)
    }
}

impl SampleTest for MaterialTest {
    fn pdf(&self, _params: &mut SampleTestParameters, dir: &Vec3) -> f32 {
        self.material.pdf(&self.incoming, dir, &self.hit)
    }

    fn sample(&self, params: &mut SampleTestParameters, rv: &Vec2) -> Option<Vec3> {
        if let Some(srec) = self.material.sample(&self.incoming, &self.hit, rv) {
            let dir = srec.wo;
            if srec.is_specular {
                params.any_specular = true;
            }
            let wo = glm::normalize(&srec.wo);
            if glm::dot(&wo, &self.hit.sn) < -1e-8 {
                params.any_below_hemisphere = true;
                return None;
            }
            return Some(dir);
        } else {
            return None;
        }
    }
}

pub struct SurfaceTest {
    surface_group: SurfaceGroupType,
    // normal: Vec3,
    // incoming: Vec3,
    // hit: HitInfo
}

impl SurfaceTest {
    // it is used in tests, but it gives me a warning : FIXME !
    #[allow(unused)]
    pub fn new(v: Value) -> (SurfaceTest, SampleTestParameters) {
        let surface_json = v["surface"].clone();
        let mf = MaterialFactory::new();
        let material = mf.create_material(surface_json["material"].clone());

        

        let mut surface_facory = SurfaceFactory::new();
        let mut surfaces_vec = Vec::new();
        if let Some(mut surface) = surface_facory.make(&surface_json.clone()) {
            surfaces_vec.append(&mut surface);
        } else {
            panic!(
                "surface of type : {} not yet supported",
                surface_json["type"]
            );
        }
         let surface_group =  SurfaceGroupType::from(LinearSurfaceGroup {surfaces: surfaces_vec });

         let test = SurfaceTest { 
            surface_group: surface_group
        };

        let name = read(&v, "name");
        let image_width = read_or(&v, "image_width", 512);
        let image_height = read_or(&v, "image_height", 256);
        let num_samples = read_or(&v, "num_samples", 50) * image_width * image_height;

        let parameters = SampleTestParameters { 
            any_specular: false, 
            any_below_hemisphere: false, 
            name: name, 
            image_width: image_width, 
            image_height: image_height, 
            num_samples: num_samples 
        };
        (test, parameters)
    }
}


impl SampleTest for SurfaceTest {
    fn pdf(&self, _params: &mut SampleTestParameters, dir: &Vec3) -> f32 {
        self.surface_group.pdf(&Vec3::zeros(), dir)
    }

    fn sample(&self, params: &mut SampleTestParameters, rv: &Vec2) -> Option<Vec3> {
        if let Some((erec, v)) = self.surface_group.sample(&Vec3::zeros(), rv){
            let dir = glm::normalize(&erec.wi);
            return Some(dir);
        }
        return None;
    }
}

impl SampleTestParameters {
    // it is used in tests, but it gives me a warning : FIXME !
    #[allow(unused)]
    fn print_more_statistics(&self) {
        if self.any_specular {
            println!("is_specular is set. It should not be.")
        }
        if self.any_below_hemisphere {
            println!(
                "Some generated directions were below the hemisphere. 
        You should check for this case and return false from sample instead."
            );
        }
    }

    // it is used in tests, but it gives me a warning : FIXME !
    #[allow(unused)]
    fn pixel_to_direction(&self, pixel: &Vec2) -> Vec3 {
        let image_width = self.image_width as f32;
        let image_height = self.image_height as f32;
        let a: Vec2 = pixel.add_scalar(0.5);
        let b: Vec2 = Vec2::new(2.0 * PI / image_width, PI / image_height);
        return spherical_coordinates_to_direction(&a.component_mul(&b));
    }

    // it is used in tests, but it gives me a warning : FIXME !
    #[allow(unused)]
    fn direction_to_pixel(&self, dir: &Vec3) -> Vec2 {
        let image_width = self.image_width as f32;
        let image_height = self.image_height as f32;
        let a = direction_to_spherical_coordinates(dir);
        let b = Vec2::new(image_width * FRAC_1_TWOPI, image_height * FRAC_1_PI);
        return a.component_mul(&b);
    }

    // it is used in tests, but it gives me a warning : FIXME !
    #[allow(unused)]
    pub fn run(&mut self, sample_test: &dyn SampleTest, target: f32, epsilon: f32) {
        println!("---------------------------------------------------------------------------\n");
        println!("Running sample test for \"{}\"\n", self.name);

        // Merge adjacent pixels to decrease noise in the histogram
        const HISTO_SUBSAMPLE: usize = 4;

        // Step 1: Evaluate pdf over the sphere and compute its integral
        let mut integral = 0.0;
        let mut pdf = Array2d::<f32>::new(
            self.image_width / HISTO_SUBSAMPLE,
            self.image_height / HISTO_SUBSAMPLE,
        );
        let pdf_size = pdf.size();
        for y in 0..pdf.size_y {
            for x in 0..pdf.size_x {
                let mut accum = 0.0;
                for sx in 0..HISTO_SUBSAMPLE {
                    for sy in 0..HISTO_SUBSAMPLE {
                        let pixel = Vec2::new(
                            (HISTO_SUBSAMPLE * x + sx) as f32,
                            (HISTO_SUBSAMPLE * y + sy) as f32,
                        );
                        let dir = self.pixel_to_direction(&pixel);
                        let sin_theta = f32::sqrt(f32::max(1.0 - dir.z * dir.z, 0.0));
                        let pixel_area = (PI / self.image_width as f32)
                            * (PI * 2.0 / self.image_height as f32)
                            * sin_theta;
                        let value = sample_test.pdf(self, &dir);
                        accum += value;
                        integral += pixel_area * value;
                    }
                }
                pdf[(x, y)] = accum / ((HISTO_SUBSAMPLE * HISTO_SUBSAMPLE) as f32);
            }
        }

        // Step 2: Generate histogram of samples
        let mut histogram = Array2d::<f32>::new(
            self.image_width / HISTO_SUBSAMPLE,
            self.image_height / HISTO_SUBSAMPLE,
        );

        let mut valid_samples = 0;
        let mut nan_or_inf = false;
        let mut rng = rand::thread_rng();
        // Progress progress2(fmt::format("Generating samples {}", num_samples), num_samples);
        for _ in 0..self.num_samples {
            if let Some(dir) = sample_test.sample(self, &Vec2::new(rng.gen(), rng.gen())) {
                if f32::is_nan(dir.x + dir.y + dir.z) || f32::is_infinite(dir.x + dir.y + dir.z) {
                    nan_or_inf = true;
                }
                // Map scattered direction to pixel in our sample histogram
                let pixel = self.direction_to_pixel(&dir) / (HISTO_SUBSAMPLE as f32);
                if pixel.x < 0.0
                    || pixel.y < 0.0
                    || pixel.x >= histogram.size_x as f32
                    || pixel.y >= histogram.size_y as f32
                {
                    continue;
                }

                // Incorporate Jacobian of spherical mapping and bin area into the sample weight
                let sin_theta = f32::sqrt(f32::max(1.0 - dir.z * dir.z, 0.0));
                let weight = (histogram.size() as f32)
                    / (PI * (2.0 * PI) * (self.num_samples as f32) * sin_theta);
                // Accumulate into histogram
                histogram[(pixel.x as usize, pixel.y as usize)] += weight;
                valid_samples += 1;
            } else {
                continue;
            }
        }

        // Now upscale our histogram and pdf
        let mut histo_fullres = Array2d::<f32>::new(self.image_width, self.image_height);
        for y in 0..histo_fullres.size_y {
            for x in 0..histo_fullres.size_x {
                histo_fullres[(x, y)] = histogram[(x / HISTO_SUBSAMPLE, y / HISTO_SUBSAMPLE)];
            }
        }

        let mut pdf_fullres = Array2d::<f32>::new(self.image_width, self.image_height);
        for y in 0..pdf_fullres.size_y {
            for x in 0..pdf_fullres.size_x {
                pdf_fullres[(x, y)] = pdf[(x / HISTO_SUBSAMPLE, y / HISTO_SUBSAMPLE)];
            }
        }

        // Step 3: For auto-exposure, compute 99.95th percentile instead of maximum for increased robustness
        let values = pdf.data.as_mut_slice();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let max_value = values[((pdf_size as f32 - 1.0) * 0.9995) as usize];
        for i in 0..pdf_size {
            if f32::is_nan(values[i]) || f32::is_infinite(values[i]) {
                nan_or_inf = true;
            }
        }

        // Generate heat maps
        // NOTE: we use get_file_resolver()[0] here to refer to the parent directory of the scene file.
        // This assumes that the calling code has prepended this directory to the front of the global resolver list
        fs::create_dir_all("tests").expect("unable to create tests dir");
        generate_heatmap(&pdf_fullres, max_value).save(format!("tests/{}-pdf.png", self.name));
        generate_heatmap(&histo_fullres, max_value)
            .save(format!("tests/{}-sampled.png", self.name));

        // Output statistics
        println!("Integral of PDF (should be close to 1): {}\n", integral);

        let sample_integral = (valid_samples as f32) / (self.num_samples as f32);
        println!(
            "{}% of samples were valid (this should be close to 100%)\n",
            sample_integral * 100.0
        );

        approx::assert_abs_diff_eq!(integral, target, epsilon = epsilon);
        approx::assert_abs_diff_eq!(sample_integral, target, epsilon = epsilon);
        approx::assert_abs_diff_eq!(sample_integral, integral, epsilon = epsilon);

        if nan_or_inf {
            println!("Some directions/PDFs contained invalid values (NaN or infinity). This should not happen. 
        Make sure you catch all corner cases in your code.")
        }
        self.print_more_statistics();
    }
}

fn generate_heatmap(density: &Array2d<f32>, max_value: f32) -> Image2d {
    let mut result = Image2d::new(density.size_x, density.size_y);

    for y in 0..density.size_y {
        for x in 0..density.size_x {
            result[(x, y)] = inferno(density[(x, y)] / max_value);
        }
    }
    return result;
}