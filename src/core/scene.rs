use nalgebra_glm::{Vec2, Vec3};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use serde_json::{Map, Value};

use crate::core::camera::PinholeCamera;
use crate::core::image2d::Image2d;
use crate::core::ray::Ray;
use crate::core::utils::{get_progress_bar, read_v_or_f, Factory};
use crate::integrators::{create_integrator, Integrator, IntegratorType};
use crate::materials::MaterialFactory;
use crate::samplers::{create_sampler, Sampler, SamplerType};
use crate::surfaces::{create_surface_group, HitInfo, Surface, SurfaceFactory, SurfaceGroupType};

pub struct Scene {
    pub surfaces: SurfaceGroupType,
    pub emitters: SurfaceGroupType,
    pub integrator: IntegratorType,
    pub sampler: SamplerType,
    camera: PinholeCamera,
    pub background: Vec3,
}

impl Scene {
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

        let sampler = create_sampler(map_json);

        // integrator
        let integrator = create_integrator(map_json);

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
                        panic!("material of type : {} not yet supported", mat["type"])
                    });
                });
        }

        // surfaces
        let Some(surfaces) = map_json.get("surfaces") else {
            panic!("No surfaces to render :(");
        };

        let mut surface_facory = SurfaceFactory { material_factory };
        let mut surfaces_vec = surfaces
            .as_array()
            .expect("Surfaces should be in an array")
            .iter()
            .flat_map(|sur| {
                surface_facory.make(sur).unwrap_or_else(|| {
                    panic!("surface of type : {} not yet supported", sur["type"])
                })
            })
            .collect();

        let surfaces = create_surface_group(map_json, &mut surfaces_vec);

        // not sure about this cloned ... FIXME!
        let mut emitters_vec = surfaces_vec
            .iter()
            .filter(|surface| surface.is_emissive())
            .cloned()
            .collect();

        let emitters = create_surface_group(&Map::new(), &mut emitters_vec);

        Scene {
            surfaces,
            emitters,
            integrator,
            sampler,
            camera,
            background,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        self.surfaces.intersect(ray)
    }

    /// Raytrace a single pixel given its position
    fn raytrace_pixel(&self, x: usize, y: usize) -> Vec3 {
        let mut sampler = self.sampler.clone();
        let sample_count = sampler.sample_count();
        let mut rng = ChaCha8Rng::seed_from_u64(sampler.seed());
        rng.set_stream((y * (self.camera.resolution.x as usize) + x) as u64);
        // Generate multiple rays for each pixel in the image
        (0..sample_count)
            .into_iter()
            .map(|_| {
                let pixel = Vec2::new(x as f32, y as f32) + sampler.next2f(&mut rng);
                let ray = self.camera.generate_ray(pixel);
                self.integrator.li(self, &mut sampler, &mut rng, &ray)
            })
            .sum::<Vec3>()
            / (sample_count as f32)
    }

    /// Raytrace a whole image
    pub fn raytrace(&self) -> Image2d {
        let mut image = Image2d::new(
            self.camera.resolution.x as usize,
            self.camera.resolution.y as usize,
        );

        println!("Rendering ...");
        let progress_bar = get_progress_bar(image.size());

        // Compute each pixel in parallel
        let img: Vec<Vec<Vec3>> = (0..image.size_y)
            .into_par_iter() // rows in parallel
            .map(|y| {
                (0..image.size_x)
                    .into_par_iter() // columns in parallel
                    .map(|x| {
                        let pixel_value = self.raytrace_pixel(x, y);
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
