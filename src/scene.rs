use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde_json::{json, Map, Value};
use std::fmt::Write;
extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};

use crate::aabb::Aabb;
use crate::camera::PinholeCamera;
use crate::image2d::Image2d;
use crate::integrators::integrator::{create_integrator, Integrator, IntegratorType};
use crate::materials::factory::MaterialFactory;
use crate::materials::material::Material;
use crate::ray::Ray;
use crate::samplers::sampler::{create_sampler, Sampler};
use crate::surfaces::accelerators::{Bvh, LinearSurfaceGroup};
use crate::surfaces::factory::SurfaceFactory;
use crate::surfaces::surface::EmitterRecord;
use crate::surfaces::surface::{HitInfo, Surface, SurfaceGroupType};
use crate::utils::{read_v_or_f, Factory};
use crate::surfaces::surface::SurfaceType;

pub struct Scene {
    pub surfaces: SurfaceGroupType,
    pub emitters: SurfaceGroupType,
    pub integrator: IntegratorType,
    sampler_value: Value,
    camera: PinholeCamera,
    pub background: Vec3,
    max_depth: i32,
    pub n_emitters: usize
}

impl Scene {
    /// parse the sampler
    pub fn get_sampler_json(map_json: Map<String, Value>) -> Value {
        if map_json.contains_key("sampler") {
            let mut sampler_json = (*map_json.get("sampler").unwrap()).clone();
            if !sampler_json.as_object().unwrap().contains_key("type") {
                println!("No sampler 'type' specified, assuming independent sampling.");
                sampler_json["type"] = serde_json::from_str("independent").unwrap();
            }
            sampler_json
        } else {
            println!("No sampler specified, defaulting to 1 spp independent sampling.");
            json!({"type" : "independent", "samples": 1})
        }
    }

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
            "max_depth".to_string(),
        ];

        for key in map_json.keys() {
            if !toplevel_fields.contains(key) {
                panic!("Unsupported field '{key}' here:\n");
            }
        }

        // camera
        let camera = PinholeCamera::new(
            scene_json
                .get("camera")
                .expect("No camera specified in scene!"),
        );

        // integrator
        let integrator = create_integrator(&scene_json);

        // scene background
        let background = read_v_or_f(&scene_json, "background");

        // materials
        let mut material_factory = MaterialFactory::new();
        if map_json.contains_key("materials") {
            map_json
                .get("materials")
                .unwrap()
                .as_array()
                .expect("Materials should be in an array")
                .iter()
                .for_each(|mat| {
                    material_factory.make(mat).expect(
                        format!("surface of type : {} not yet supported", mat["type"]).as_str(),
                    );
                });
        }

        // surfaces
        let mut surface_facory = SurfaceFactory { material_factory };
        let mut surfaces_vec = Vec::new();
        if map_json.contains_key("surfaces") {
            for surface_json in map_json.get("surfaces").unwrap().as_array().unwrap() {
                if let Some(mut surface) = surface_facory.make(surface_json) {
                    surfaces_vec.append(&mut surface);
                } else {
                    panic!(
                        "surface of type : {} not yet supported",
                        surface_json["type"]
                    );
                }
            }
        }
        // not sure about this cloned ... FIXME!
        let emitters_vec: Vec<SurfaceType> = surfaces_vec
            .iter()
            .filter(|x| x.is_emissive())
            .cloned()
            .collect();

        let n_emitters = emitters_vec.len();

        let emitters = SurfaceGroupType::from(LinearSurfaceGroup {
            surfaces: emitters_vec,
        });

        // create the scene-wide acceleration structure so we can put other surfaces into it
        let surfaces = if map_json.contains_key("accelerator") {
            SurfaceGroupType::from(Bvh::new(&mut surfaces_vec))
        } else {
            // default to a naive linear accelerator
            SurfaceGroupType::from(LinearSurfaceGroup {
                surfaces: surfaces_vec,
            })
        };

        let max_depth: i32 = 64;

        Scene {
            integrator: integrator,
            emitters: emitters,
            sampler_value: Scene::get_sampler_json((*map_json).clone()),
            surfaces: surfaces,
            camera: camera,
            background: background,
            max_depth: max_depth,
            n_emitters: n_emitters
        }
    }

    fn recursive_color(&self, ray: &Ray, depth: i32) -> Vec3 {
        const BLACK: Vec3 = Vec3::new(0.0, 0.0, 0.0);

        if let Some(hit) = self.intersect(ray) {
            let emitted = hit.mat.emmitted(ray, &hit).unwrap_or(BLACK);
            if depth < self.max_depth {
                if let Some((attenuation, scattered)) = hit.mat.scatter(ray, &hit) {
                    return emitted
                        + attenuation.component_mul(&self.recursive_color(&scattered, depth + 1));
                }
            }
            return emitted;
        } else {
            return self.background;
        }
    }

    pub fn raytrace(&self) -> Image2d {
        let mut image = Image2d::new(
            self.camera.resolution.x as usize,
            self.camera.resolution.y as usize,
        );
        let mut sampler = create_sampler(&self.sampler_value);
        let sample_count = sampler.sample_count();

        println!("Rendering ...");
        let progress_bar = ProgressBar::new(image.size() as u64);
        progress_bar.set_style(
            ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

        // Generate a ray for each pixel in the ray image
        for y in 0..image.size_y {
            for x in 0..image.size_x {
                image[(x, y)] = (0..sample_count)
                    .into_iter()
                    .map(|_| {
                        let pixel = Vec2::new(x as f32, y as f32) + sampler.next2f();
                        let ray = self.camera.generate_ray(&pixel);
                        if self.integrator.is_integrator() {
                            self.integrator.li(self, &mut sampler, &ray, 0)
                        } else {
                            self.recursive_color(&ray, 0)
                        }
                    })
                    .sum::<Vec3>()
                    / (sample_count as f32);

                progress_bar.inc(1);
            }
        }

        println!("Rendering time : {:?}", progress_bar.elapsed());
        return image;
    }
}

impl Surface for Scene {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        self.surfaces.intersect(ray)
    }

    fn bounds(&self) -> Aabb {
        unimplemented!()
    }

    fn pdf(&self, _o: &Vec3, _dir: &Vec3) -> f32 {
        unimplemented!()
    }

    fn sample(&self, _o: &Vec3, _rv: &Vec2) -> Option<(EmitterRecord, Vec3)> {
        unimplemented!()
    }

    fn is_emissive(&self) -> bool {
        unimplemented!()
    }
}
