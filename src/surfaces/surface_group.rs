use nalgebra_glm::{Vec2, Vec3};

use crate::core::ray::Ray;
use crate::surfaces::{EmitterRecord, HitInfo, Surface, SurfaceType};

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
                ray.max_t = hit.t;
                option_hit.replace(hit);
            }
        }
        option_hit
    }

    fn pdf(&self, _o: &Vec3, _dir: &Vec3) -> f32 {
        // must multiply this by the child pdf
        let n_sufaces = self.surfaces.len() as f32;
        1.0 / n_sufaces
    }

    fn pdf_child(&self, o: &Vec3, dir: &Vec3, rv: f32) -> f32 {
        let index = (rv * (self.surfaces.len() as f32)) as usize;
        self.surfaces[index].pdf(o, dir)
    }

    fn sample(&self, origin: &Vec3, rv: Vec2) -> Option<EmitterRecord> {
        let index = (rv.x * (self.surfaces.len() as f32)) as usize;
        self.surfaces[index].sample(origin, rv)
    }

    fn sample_from_group(&self, o: &Vec3, rv: Vec2, rv1: f32) -> Option<EmitterRecord> {
        let index = (rv1 * (self.surfaces.len() as f32)) as usize;
        self.surfaces[index].sample(o, rv)
    }

    fn is_emissive(&self) -> bool {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::sample_test::SurfaceTest;
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

        let (test, mut parameters) = SurfaceTest::new(&v);
        parameters.run(&test, 1.0, 1e-2);
    }
}
