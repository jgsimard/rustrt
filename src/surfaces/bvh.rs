extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use indicatif::ProgressBar;
use partition::partition;

use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::surfaces::surface::{EmitterRecord, HitInfo, Surface, SurfaceType};
use crate::utils::get_progress_bar;

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
                    ray.max_t = hit.t;
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
    fn sample(&self, _o: &Vec3, _rv: Vec2) -> Option<EmitterRecord> {
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
        let progress_bar = get_progress_bar(surfaces.len());
        let bvh = Bvh::new_node(surfaces.as_mut_slice(), max_leaf_size, &progress_bar);
        println!("Building BVH... Done in {:?}", progress_bar.elapsed());
        bvh
    }

    fn new_node(surfaces: &mut [SurfaceType], max_leaf_size: usize, pb: &ProgressBar) -> Bvh {
        let n_surfaces = surfaces.len();
        if n_surfaces <= max_leaf_size {
            // println!("depth : {}, number of children {}", depth, n_surfaces);
            pb.inc(n_surfaces as u64);
            let mut bbox = Aabb::new();
            for child in surfaces.iter() {
                bbox.enclose(&child.bounds());
            }
            return Bvh {
                bbox,
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
        let children = vec![
            SurfaceType::from(Bvh::new_node(left, max_leaf_size, pb)),
            SurfaceType::from(Bvh::new_node(right, max_leaf_size, pb)),
        ];

        let mut bbox = Aabb::new();
        for child in children.iter() {
            bbox.enclose(&child.bounds());
        }

        Bvh { bbox, children }
    }
}
