extern crate nalgebra_glm as glm;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;


use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use std::rc::Rc;

pub struct LinearSurfaceGroup {
    pub surfaces: Vec<Rc<dyn Surface>>,
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
}

pub struct Bvh {
    bbox: Aabb,
    children: Vec<Rc<dyn Surface>>,
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
}
impl Bvh {
    pub fn new(surfaces: &mut Vec<Rc<dyn Surface>>) -> Bvh {
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

    pub fn new_node(surfaces: &mut [Rc<dyn Surface>], depth: i32, max_leaf_size: usize, pb: &ProgressBar) -> Bvh {
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

        // Equal method
        let center = |a: &Rc<dyn Surface>| a.bounds().center()[split_axis];
        let mid = n_surfaces / 2;
        surfaces.select_nth_unstable_by(mid, |a, b| center(a).total_cmp(&center(b)));
        let (left, right) = surfaces.split_at_mut(mid);

        // add the two children => recursion
        let mut children: Vec<Rc<dyn Surface>> = Vec::new();
        children.push(Rc::new(Bvh::new_node(left, depth + 1, max_leaf_size, pb)));
        children.push(Rc::new(Bvh::new_node(right, depth + 1, max_leaf_size, pb)));

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
