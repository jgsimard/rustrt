extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use partition::partition;
use rand::Rng;
use std::fmt::Write;

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::surfaces::surface::{EmitterRecord, HitInfo, Surface, SurfaceType};

#[derive(Debug, PartialEq, Clone)]
pub struct LinearSurfaceGroup {
    pub surfaces: Vec<SurfaceType>,
}

impl Surface for LinearSurfaceGroup {
    fn intersect(&self, ray_: &Ray) -> Option<HitInfo> {
        let mut ray: Ray = (*ray_).clone();
        let mut option_hit: Option<HitInfo> = None;

        for surface in &self.surfaces {
            if let Some(hit) = surface.intersect(&ray) {
                ray.maxt = hit.t;
                option_hit.replace(hit);
            }
        }
        option_hit
    }

    fn pdf(&self, _o: &Vec3, _dir: &Vec3) -> f32 {
        // must multiply this by the child pdf
        let n_sufaces = self.surfaces.len() as f32;
        return 1.0 / n_sufaces;
        // let mut pdf = 0.0;
        // if let Some(_hit) = self.intersect(&Ray::new(*o, *dir)) {
        //     let n_sufaces = self.surfaces.len() as f32;
        //     for surface in self.surfaces.iter() {
        //         pdf += surface.pdf(o, dir) / n_sufaces;
        //     }
        // }
        // pdf
    }
    fn sample(&self, o: &Vec3, rv: &Vec2) -> Option<(EmitterRecord, Vec3)> {
        let mut rng = rand::thread_rng();
        self.surfaces[rng.gen_range(0..self.surfaces.len())].sample(o, rv)
    }

    fn is_emissive(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bvh {
    bbox: Aabb,
    children: Vec<SurfaceType>,
}

impl Surface for Bvh {
    fn intersect(&self, ray_: &Ray) -> Option<HitInfo> {
        let mut option_hit: Option<HitInfo> = None;
        if self.bbox.intersect(ray_) {
            let mut ray = ray_.clone();

            for child in &self.children {
                if let Some(hit) = child.intersect(&ray) {
                    ray.maxt = hit.t;
                    option_hit.replace(hit);
                }
            }
        }
        option_hit
    }

    fn bounds(&self) -> Aabb {
        self.bbox.clone()
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
impl Bvh {
    pub fn new(surfaces: &mut Vec<SurfaceType>) -> Bvh {
        let max_leaf_size = 3;
        println!("Building BVH...");
        let progress_bar = ProgressBar::new(surfaces.len() as u64);
        progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
            .progress_chars("#>-"));

        let bvh = Bvh::new_node(surfaces.as_mut_slice(), 0, max_leaf_size, &progress_bar);
        println!("Building BVH... Done in {:?}", progress_bar.elapsed());
        return bvh;
    }

    fn new_node(
        surfaces: &mut [SurfaceType],
        depth: i32,
        max_leaf_size: usize,
        pb: &ProgressBar,
    ) -> Bvh {
        let n_surfaces = surfaces.len();
        if n_surfaces <= max_leaf_size {
            // println!("depth : {}, number of children {}", depth, n_surfaces);
            pb.inc(n_surfaces as u64);
            let mut bbox = Aabb::new();
            for child in surfaces.iter() {
                bbox.enclose(&child.bounds());
            }
            return Bvh {
                bbox: bbox,
                children: Vec::from(surfaces),
            };
        }
        // chose the slit_axis that has the biggest range
        let mut max = glm::vec3(f32::MIN, f32::MIN, f32::MIN);
        let mut min = glm::vec3(f32::MAX, f32::MAX, f32::MAX);

        for s in surfaces.iter() {
            let c = s.bounds().center();
            min = glm::min2(&min, &c);
            max = glm::max2(&max, &c);
        }

        let (split_axis, _) = (max - min).argmax();
        let center = |a: &SurfaceType| a.bounds().center()[split_axis];

        // Equal method
        // let mid = n_surfaces / 2;
        // surfaces.select_nth_unstable_by(mid, |a, b| center(a).total_cmp(&center(b)));
        // let (left, right) = surfaces.split_at_mut(mid);

        // Middle Method
        let middle = (max + min)[split_axis] / 2.0;
        let (left, right) = partition(surfaces, |x| center(x) >= middle);

        // add the two children => recursion
        let mut children: Vec<SurfaceType> = Vec::new();
        children.push(SurfaceType::from(Bvh::new_node(
            left,
            depth + 1,
            max_leaf_size,
            pb,
        )));
        children.push(SurfaceType::from(Bvh::new_node(
            right,
            depth + 1,
            max_leaf_size,
            pb,
        )));

        let mut bbox = Aabb::new();
        for child in children.iter() {
            bbox.enclose(&child.bounds());
        }

        Bvh {
            bbox: bbox,
            children: children,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::SurfaceTest;
    use serde_json::json;
    #[test]
    fn surface_group_monte_carlo() {
        let v = json!({
            "type": "sample_surface",
            "name": "group",
            "surfaces": {
                "type": "group",
                "children": [
                    {
                        "type": "triangle",
                        "positions": [
                            [
                                -1.5, 0.2, -1.0
                            ],
                            [
                                2.5, 0.375, -1.0
                            ],
                            [
                                -0.5, 0.2, 1.0
                            ]
                        ],
                        "material": {
                            "type": "lambertian",
                            "albedo": 1.0
                        }
                    }, {
                        "type": "quad",
                        "size": 1.0,
                        "transform": {
                            "o": [
                                0, 0, 1
                            ],
                            "x": [
                                1, 1, 0
                            ],
                            "y": [0, 1, 1]
                        },
                        "material": {
                            "type": "lambertian",
                            "albedo": 1.0
                        }
                    }, {
                        "type": "sphere",
                        "radius": 3.0,
                        "transform": {
                            "o": [0, 2.4, 3.2]
                        },
                        "material": {
                            "type": "lambertian",
                            "albedo": 1.0
                        }
                    }, {
                        "type": "mesh",
                        "filename": "assets/cube.obj",
                        "material": {
                            "type": "lambertian",
                            "albedo": 1.0
                        },
                        "transform": [
                            {
                                "translate": [0.0, -0.6, 0.2]
                            }
                        ]
                    }
                ]
            }
        });

        let (test, mut parameters) = SurfaceTest::new(v);
        parameters.run(&test, 1.0, 1e-2);
    }
}
