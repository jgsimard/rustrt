use nalgebra_glm::{cross, normalize, Vec3};

/// `OrthoNormal` Basis
pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn build_from_w(normal: &Vec3) -> Self {
        let w = normalize(normal);
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = normalize(&cross(&w, &a));
        let u = cross(&w, &v);
        Onb { axis: [u, v, w] }
    }

    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        a.x * self.u() + a.y * self.v() + a.z * self.w()
    }
}
