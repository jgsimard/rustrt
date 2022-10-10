use crate::ray::Ray;
use crate::surfaces::surface::{HitInfo, Surface};
use std::rc::Rc;

pub struct SurfaceGroup {
    pub surfaces: Vec<Rc<dyn Surface>>,
    // bounds: Box3
}

// TODO : CHANGE THIS< THIS IS HORRIBLE
impl Surface for SurfaceGroup {
    fn intersect(&self, ray: &Ray) -> Option<HitInfo> {
        let mut ray_mut: Ray = (*ray).clone();
        let mut hit_anything = false;
        let mut hit_out = HitInfo::empty();

        for surface in &self.surfaces {
            if let Some(hit) = surface.intersect(&ray_mut) {
                hit_anything = true;
                ray_mut.maxt = hit.t;
                hit_out = hit;
            }
        }

        if hit_anything {
            // Some(hit)
            Some(hit_out)
        } else {
            None
        }
    }
}

impl SurfaceGroup {
    pub fn new() -> SurfaceGroup {
        SurfaceGroup {
            surfaces: Vec::new(),
        }
    }
    pub fn add_child(&mut self, surface: Rc<dyn Surface>) {
        self.surfaces.push(surface.clone())
    }

    pub fn add_to_parent(&self) {}
}
