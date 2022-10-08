use nalgebra::Vector3;
// use nalgebra::Unit;

// TODO : Use Unit for direction

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub mint: f32,
    pub maxt: f32,
}

impl Ray {
    // pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, mint: f32, maxt: f32) -> Ray {
    //     Ray {
    //         origin: origin,
    //         direction: direction,
    //         mint: mint,
    //         maxt: maxt
    //     }
    // }

    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
            mint: 0.0001, // TODO : maybe change this
            maxt: f32::INFINITY,
        }
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }
}
