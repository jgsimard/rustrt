extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use indicatif::ProgressBar;
use partition::partition_index;
use serde::{Deserialize, Serialize};

use crate::core::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::utils::get_progress_bar;
use crate::surfaces::surface::{EmitterRecord, HitInfo, Surface, SurfaceType};

#[derive(Serialize, Deserialize)]
pub enum SplitMethod {
    Equal,
    Middle,
    Sah,
}

/// small modification of partition to include edge case of partition_index at start or end of array
pub fn partition<T, P>(data: &mut [T], predicate: P, max_leaf_size: usize) -> (&mut [T], &mut [T])
where
    P: Fn(&T) -> bool,
{
    let mut idx = partition_index(data, predicate);
    if idx == 0 {
        idx += max_leaf_size;
    } else if idx == data.len() {
        idx -= max_leaf_size;
    }
    data.split_at_mut(idx)
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
    pub fn new(surfaces: &mut Vec<SurfaceType>, split_method: &SplitMethod) -> Bvh {
        let max_leaf_size = 3;
        println!("Building BVH...");
        let progress_bar = get_progress_bar(surfaces.len());
        let bvh = Bvh::new_node(
            surfaces.as_mut_slice(),
            max_leaf_size,
            split_method,
            &progress_bar,
        );
        println!("Building BVH... Done in {:?}", progress_bar.elapsed());
        bvh
    }

    fn new_node(
        surfaces: &mut [SurfaceType],
        max_leaf_size: usize,
        split_method: &SplitMethod,
        pb: &ProgressBar,
    ) -> Bvh {
        let n_surfaces = surfaces.len();
        if n_surfaces <= max_leaf_size {
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

        let (left, right) = match split_method {
            SplitMethod::Equal => {
                let mid = n_surfaces / 2;
                surfaces.select_nth_unstable_by(mid, |a, b| center(a).total_cmp(&center(b)));
                surfaces.split_at_mut(mid)
            }
            SplitMethod::Middle => {
                let middle = (max + min)[split_axis] / 2.0;
                partition(surfaces, |x| center(x) >= middle, max_leaf_size)
            }
            SplitMethod::Sah => todo!(),
        };

        // add the two children => recursion
        let (c1, c2) = rayon::join(
            || Bvh::new_node(left, max_leaf_size, split_method, pb),
            || Bvh::new_node(right, max_leaf_size, split_method, pb),
        );
        let children = vec![SurfaceType::from(c1), SurfaceType::from(c2)];

        let mut bbox = Aabb::new();
        for child in &children {
            bbox.enclose(&child.bounds());
        }

        Bvh { bbox, children }
    }
}
