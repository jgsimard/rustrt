use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use nalgebra::{Vector2, Vector3};
use serde_json::{from_value, json, Value};
use std::fmt::Write;
use std::rc::Rc;

use crate::camera::PinholeCamera;
use crate::image2d::Image2d;
use crate::ray::Ray;
use crate::surface::{make_surface, HitInfo, Surface, SurfaceGroup};

pub struct Scene {
    pub surfaces: SurfaceGroup,
    // pub emitters: SurfaceGroup,
    // pub integrator: Rc<dyn Integrator>,
    // pub sampler: Rc<dyn Sampler>,
    pub camera: PinholeCamera,
    pub background: Vector3<f32>,
    pub num_samples: u32,
    pub max_depth: i32,
}

fn make_sampler(m: serde_json::Value) -> i32 {
    -1
}

fn make_integrator(v: &Value) -> i32 {
    -1
}

fn make_accelerator(v: &Value) -> i32 {
    -1
}

impl Scene {
    pub fn new(scene_json: Value) -> Scene {
        println!("Parsing...");
        let map_json = scene_json.as_object().unwrap();

        // check if all fields are expeted ones (no all implemented)
        let toplevel_fields = [
            "integrator".to_string(),
            "media".to_string(),
            "materials".to_string(),
            "surfaces".to_string(),
            "accelerator".to_string(),
            "camera".to_string(),
            "sampler".to_string(),
            "background".to_string(),
            "num_samples".to_string(),
            "max_depth".to_string(),
        ];

        for key in map_json.keys() {
            if !toplevel_fields.contains(key) {
                panic!("Unsupported field '{key}' here:\n");
            }
        }

        // Utility function
        let read_vector3 = |v: &Value| from_value::<Vector3<f32>>(v.clone()).unwrap();

        //
        // parse the camera
        //
        let camera = PinholeCamera::new(
            scene_json
                .get("camera")
                .expect("No camera specified in scene!"),
        );

        // //
        // // parse the sampler
        // //
        // let sampler = if map_json.contains_key("sampler"){
        //     let sampler_json = map_json.get("sampler").unwrap().as_object().unwrap();
        //     if !sampler_json.contains_key("type"){
        //         println!("No sampler 'type' specified, assuming independent sampling.");
        //         sampler_json["type"] = serde_json::from_str("independent").unwrap();
        //     }
        //     // make_sampler(sampler_json.unwrap())
        //     -1
        // } else {
        //     println!("No sampler specified, defaulting to 1 spp independent sampling.");
        //     make_sampler(json!({"type" : "independent", "samples": 1}))
        // };

        // //
        // // parse the integrator
        // //

        // let integrator = if map_json.contains_key("integrator"){
        //     make_integrator(map_json.get("integrator").unwrap())
        // }else{
        //     make_integrator(&json!({}))
        // };

        //
        // create the scene-wide acceleration structure so we can put other surfaces into it
        //
        let surfaces = if map_json.contains_key("accelerator") {
            make_accelerator(map_json.get("accelerator").unwrap())
        } else {
            // default to a naive linear accelerator
            make_accelerator(&json!({}))
        };

        //
        // parse scene background
        //
        let background = map_json
            .get("background")
            .map_or(Vector3::zeros(), read_vector3);

        // //
        // // parse materials
        // //
        // if (j.contains("materials"))
        // {
        //     for (auto &m : j["materials"])
        //     {
        //         auto material = DartsFactory<Material>::create(m);
        //         DartsFactory<Material>::register_instance(check_key("name", "material", m), material);
        //     }
        // }

        //
        // parse surfaces
        //
        if map_json.contains_key("surfaces") {
            for sur in map_json.get("surfaces").unwrap().as_array().unwrap() {
                let surface = make_surface(sur);
                // DartsFactory<Surface>::create(s);
                // surface->add_to_parent(this, surface, j);
            }
        }

        //
        // parse num_samples and max_depth
        //
        let num_samples: u32 =
            from_value(map_json.get("num_samples").ok_or(8).unwrap().clone()).unwrap();
        let max_depth: i32 =
            from_value(map_json.get("max_depth").ok_or(64).unwrap().clone()).unwrap();

        Scene {
            surfaces: SurfaceGroup {
                surfaces: Vec::new(),
            },
            camera: camera,
            background: background,
            num_samples: num_samples,
            max_depth: max_depth,
        }
    }
    pub fn add_child(&mut self, surface: Rc<dyn Surface>) {
        self.surfaces.add_child(surface);
    }

    pub fn add_to_parent(&self) {}

    fn recursive_color(&self, ray: &Ray, depth: i32) -> Vector3<f32> {
        const BLACK: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

        if let Some(hit) = self.intersect(ray) {
            if depth < self.max_depth {
                if let Some((attenuation, scattered)) = hit.mat.scatter(ray, &hit) {
                    return attenuation.component_mul(&self.recursive_color(&scattered, depth + 1));
                }
                return BLACK;
            } else {
                return BLACK;
            }
        } else {
            return self.background;
        }
    }

    pub fn raytrace(&self) -> Image2d {
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
            for y in 0..image.size_y {
                for x in 0..image.size_x {
                    let mut color = Vector3::new(0.0, 0.0, 0.0);
                    for _ in 0..sample_count {
                        // rays_traced.inc(1);
                        let ray = self
                            .camera
                            .generate_ray(&Vector2::new((x as f32) + 0.5, (y as f32) + 0.5));
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
