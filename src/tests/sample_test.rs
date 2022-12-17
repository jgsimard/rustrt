use nalgebra_glm::{dot, normalize, Vec2, Vec3};
use rand::Rng;
use serde_json::{Map, Value};
use std::f32::consts::FRAC_1_PI;
use std::f32::consts::PI;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::core::image2d::{Array2d, Image2d};
use crate::core::utils::{
    direction_to_spherical_coordinates, inferno, read, read_or, spherical_coordinates_to_direction,
    Factory, FRAC_1_TWOPI,
};
use crate::materials::{Material, MaterialFactory, MaterialType};
use crate::surfaces::{create_surface_group, HitInfo, Surface, SurfaceFactory, SurfaceGroupType};

pub trait SampleTest {
    fn sample(&self, params: &mut SampleTestParameters, rv: Vec2, rv1: f32) -> Option<Vec3>;
    fn pdf(&self, params: &mut SampleTestParameters, dir: &Vec3, rv: f32) -> f32;
}

pub struct MaterialTest {
    material: Arc<MaterialType>,
    // normal: Vec3,
    incoming: Vec3,
    hit: HitInfo,
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
    pub fn new(v: &Value) -> (MaterialTest, SampleTestParameters) {
        let mf = MaterialFactory::new();
        let material = mf.create_material(&v["material"]);
        let normal = normalize(&read(v, "normal"));
        let incoming = normalize(&read_or(v, "incoming", Vec3::new(0.25, 0.0, -1.0)));
        let hit = HitInfo {
            t: 1.0,
            p: Vec3::zeros(),
            gn: normal,
            sn: normal,
            uv: Vec2::new(0.5, 0.5),
            mat: material.clone(),
        };
        let name = read(v, "name");
        let image_width = read_or(v, "image_width", 512);
        let image_height = read_or(v, "image_height", 256);
        let num_samples = read_or(v, "num_samples", 50) * image_width * image_height;

        let test = MaterialTest {
            material,
            // normal: normal,
            incoming,
            hit,
        };
        let parameters = SampleTestParameters {
            any_specular: false,
            any_below_hemisphere: false,
            name,
            image_width,
            image_height,
            num_samples,
        };
        (test, parameters)
    }
}

impl SampleTest for MaterialTest {
    fn pdf(&self, _params: &mut SampleTestParameters, dir: &Vec3, _rv: f32) -> f32 {
        self.material.pdf(&self.incoming, dir, &self.hit)
    }

    fn sample(&self, params: &mut SampleTestParameters, rv: Vec2, _rv1: f32) -> Option<Vec3> {
        if let Some(srec) = self.material.sample(&self.incoming, &self.hit, rv) {
            let dir = srec.wo;
            if srec.is_specular {
                params.any_specular = true;
            }
            let wo = normalize(&srec.wo);
            if dot(&wo, &self.hit.sn) < -1e-8 {
                params.any_below_hemisphere = true;
                return None;
            }
            Some(dir)
        } else {
            None
        }
    }
}

pub struct SurfaceTest {
    surface_group: SurfaceGroupType,
}

impl SurfaceTest {
    pub fn new(v: &Value) -> (SurfaceTest, SampleTestParameters) {
        let m = v.as_object().unwrap();
        let surface_json = if m.contains_key("surface") {
            v["surface"].clone()
        } else if m.contains_key("surfaces") {
            v["surfaces"].clone()
        } else {
            println!("{}", v);
            panic!("NOOOOO");
        };

        let mut surface_facory = SurfaceFactory {
            material_factory: MaterialFactory::new(),
        };
        let mut surfaces_vec = Vec::new();
        if let Some(mut surface) = surface_facory.make(&surface_json) {
            surfaces_vec.append(&mut surface);
        } else {
            panic!(
                "surface of type : {} not yet supported",
                surface_json["type"]
            );
        }
        let surface_group = create_surface_group(&Map::new(), &mut surfaces_vec);

        let test = SurfaceTest { surface_group };

        let name = read(v, "name");
        let image_width = read_or(v, "image_width", 512);
        let image_height = read_or(v, "image_height", 256);
        let num_samples = read_or(v, "num_samples", 50) * image_width * image_height;

        let parameters = SampleTestParameters {
            any_specular: false,
            any_below_hemisphere: false,
            name,
            image_width,
            image_height,
            num_samples,
        };
        (test, parameters)
    }
}

impl SampleTest for SurfaceTest {
    fn pdf(&self, _params: &mut SampleTestParameters, dir: &Vec3, rv: f32) -> f32 {
        self.surface_group.pdf_child(&Vec3::zeros(), dir, rv)
    }

    fn sample(&self, _params: &mut SampleTestParameters, rv: Vec2, rv1: f32) -> Option<Vec3> {
        let erec = self
            .surface_group
            .sample_from_group(&Vec3::zeros(), rv, rv1)?;
        let dir = normalize(&erec.wi);
        Some(dir)
    }
}

impl SampleTestParameters {
    fn print_more_statistics(&self) {
        if self.any_specular {
            println!("is_specular is set. It should not be.");
        }
        if self.any_below_hemisphere {
            println!(
                "Some generated directions were below the hemisphere. 
        You should check for this case and return false from sample instead."
            );
        }
    }

    fn pixel_to_direction(&self, pixel: Vec2) -> Vec3 {
        let image_width = self.image_width as f32;
        let image_height = self.image_height as f32;
        let a: Vec2 = pixel.add_scalar(0.5);
        let b: Vec2 = Vec2::new(2.0 * PI / image_width, PI / image_height);
        spherical_coordinates_to_direction(a.component_mul(&b))
    }

    fn direction_to_pixel(&self, dir: &Vec3) -> Vec2 {
        let image_width = self.image_width as f32;
        let image_height = self.image_height as f32;
        let a = direction_to_spherical_coordinates(dir);
        let b = Vec2::new(image_width * FRAC_1_TWOPI, image_height * FRAC_1_PI);
        a.component_mul(&b)
    }

    #[allow(clippy::cast_precision_loss)] // TODO
    #[allow(clippy::cast_sign_loss)] // TODO
    pub fn run(&mut self, sample_test: &dyn SampleTest, target: f32, epsilon: f32) {
        // Merge adjacent pixels to decrease noise in the histogram
        const HISTO_SUBSAMPLE: usize = 4;
        const NB_SAMPLES: usize = 10;
        let mut rng = rand::thread_rng();

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
                        for _ in 0..NB_SAMPLES {
                            let pixel = Vec2::new(
                                (HISTO_SUBSAMPLE * x + sx) as f32,
                                (HISTO_SUBSAMPLE * y + sy) as f32,
                            );
                            let dir = self.pixel_to_direction(pixel);
                            let sin_theta = f32::sqrt(f32::max(1.0 - dir.z * dir.z, 0.0));
                            let pixel_area = (PI / self.image_width as f32)
                                * (PI * 2.0 / self.image_height as f32)
                                * sin_theta;
                            let value = sample_test.pdf(self, &dir, rng.gen());
                            accum += value / (NB_SAMPLES as f32);
                            integral += pixel_area * value / (NB_SAMPLES as f32);
                        }
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
        for _ in 0..self.num_samples {
            if let Some(dir) = sample_test.sample(self, Vec2::new(rng.gen(), rng.gen()), rng.gen())
            {
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
        if values
            .iter()
            .any(|value| value.is_nan() || value.is_infinite())
        {
            nan_or_inf = true;
        }

        // Generate heat maps
        fs::create_dir_all("tests").expect("unable to create tests dir");
        generate_heatmap(&pdf_fullres, max_value)
            .save(&PathBuf::from(format!("tests/{}-pdf.png", self.name)));
        generate_heatmap(&histo_fullres, max_value)
            .save(&PathBuf::from(format!("tests/{}-sampled.png", self.name)));

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

        assert!(!nan_or_inf, "Some directions/PDFs contained invalid values (NaN or infinity). This should not happen. 
        Make sure you catch all corner cases in your code.");
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
    result
}
