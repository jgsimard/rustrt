use nalgebra_glm::{Vec2, Vec3};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde_json::{json, Map, Value};

use crate::core::aabb::Aabb;
use crate::core::camera::PinholeCamera;
use crate::core::image2d::Image2d;
use crate::core::ray::Ray;
use crate::core::utils::{get_progress_bar, read_v_or_f, Factory};
use crate::integrators::{create_integrator, Integrator, IntegratorType};
use crate::materials::MaterialFactory;
use crate::samplers::{create_sampler, Sampler};
use crate::surfaces::{
    create_surface_group, EmitterRecord, HitInfo, Surface, SurfaceFactory, SurfaceGroupType,
};

pub struct Scene {
    pub surfaces: SurfaceGroupType,
    pub emitters: SurfaceGroupType,
    pub integrator: IntegratorType,
    sampler_value: Value,
    camera: PinholeCamera,
    pub background: Vec3,
}

impl Scene {
    /// parse the sampler
    pub fn get_sampler_json(map_json: &Map<String, Value>) -> Value {
        if let Some(sampler_value) = map_json.get("sampler") {
            let mut sampler_map = sampler_value.as_object().unwrap().clone();
            if !sampler_map.contains_key("type") {
                println!("No sampler 'type' specified, assuming independent sampling.");
                sampler_map.insert("type".to_string(), json!("independent"));
            }
            if !sampler_map.contains_key("samples") {
                println!("Number of samples is not specified, assuming 1.");
                sampler_map.insert("samples".to_string(), json!(1));
            }
            serde_json::to_value(sampler_map).unwrap()
        } else {
            println!("No sampler specified, defaulting to 1 spp independent sampling.");
            json!({"type" : "independent", "samples": 1})
        }
    }

    pub fn new(scene_json: &Value) -> Scene {
        println!("Parsing...");
        let map_json = scene_json.as_object().unwrap();

        // check if all fields are expeted ones (no all implemented)
        let toplevel_fields = [
            "integrator",
            "media",
            "materials",
            "surfaces",
            "accelerator",
            "camera",
            "sampler",
            "background",
        ];

        for key in map_json.keys() {
            assert!(
                toplevel_fields.contains(&key.as_str()),
                "Unsupported field '{key}' here:\n"
            );
        }

        // camera
        let camera = PinholeCamera::new(
            scene_json
                .get("camera")
                .expect("No camera specified in scene!"),
        );

        // integrator
        let integrator = create_integrator(scene_json);

        // scene background
        // TODO replace by let background = create_texture(&scene_json, "background")
        let background = read_v_or_f(scene_json, "background");

        // materials
        let mut material_factory = MaterialFactory::new();
        if let Some(materials) = map_json.get("materials") {
            materials
                .as_array()
                .expect("Materials should be in an array")
                .iter()
                .for_each(|mat| {
                    material_factory.make(mat).unwrap_or_else(|| {
                        panic!("surface of type : {} not yet supported", mat["type"])
                    });
                });
        }

        // surfaces
        let mut surface_facory = SurfaceFactory { material_factory };
        let mut surfaces_vec = Vec::new();
        if let Some(surfaces) = map_json.get("surfaces") {
            for surface_json in surfaces.as_array().unwrap() {
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
        let surfaces = create_surface_group(map_json, &mut surfaces_vec);

        // not sure about this cloned ... FIXME!
        let mut emitters_vec = surfaces_vec
            .iter()
            .filter(|x| x.is_emissive())
            .cloned()
            .collect();

        let emitters = create_surface_group(&Map::new(), &mut emitters_vec);

        Scene {
            integrator,
            emitters,
            sampler_value: Scene::get_sampler_json(map_json),
            surfaces,
            camera,
            background,
        }
    }

    fn raytrace_pixel(&self, x: usize, y: usize, size_x: usize) -> Vec3 {
        let mut sampler = create_sampler(&self.sampler_value);
        let sample_count = sampler.sample_count();
        let mut rng = ChaCha8Rng::seed_from_u64(sampler.seed());
        rng.set_stream((y * size_x + x) as u64);
        (0..sample_count)
            .into_iter()
            .map(|_| {
                let pixel = Vec2::new(x as f32, y as f32) + sampler.next2f(&mut rng);
                let ray = self.camera.generate_ray(&pixel);
                self.integrator.li(self, &mut sampler, &mut rng, &ray)
            })
            .sum::<Vec3>()
            / (sample_count as f32)
    }

    pub fn raytrace(&self) -> Image2d {
        let mut image = Image2d::new(
            self.camera.resolution.x as usize,
            self.camera.resolution.y as usize,
        );

        println!("Rendering ...");
        let progress_bar = get_progress_bar(image.size());

        // Generate multiple rays for each pixel in the image
        let size_x = image.size_x;
        let img: Vec<Vec<Vec3>> = (0..image.size_y)
            .into_par_iter() // rows in parallel
            .map(|y| {
                (0..image.size_x)
                    .into_par_iter() // columns in parallel
                    .map(|x| {
                        let pixel_value = self.raytrace_pixel(x, y, size_x);
                        progress_bar.inc(1);
                        pixel_value
                    })
                    .collect()
            })
            .collect();

        for (y, row) in img.into_iter().enumerate() {
            for (x, p) in row.into_iter().enumerate() {
                image[(x, y)] = p;
            }
        }

        println!("Rendering time : {:?}", progress_bar.elapsed());
        image
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

    fn sample(&self, _o: &Vec3, _rv: Vec2) -> Option<EmitterRecord> {
        unimplemented!()
    }

    fn is_emissive(&self) -> bool {
        unimplemented!()
    }
}
