use nalgebra_glm::Vec2;
use rand::Rng;

use crate::samplers::Sampler;

/// Independent sampling
///
/// returns independent uniformly distributed random numbers on \f$[0, 1)^2\f$
#[derive(Debug, Clone)]
pub struct IndependentSampler {
    base_seed: u64,
    sample_count: i32,
    current_sample: i32,
    current_dimension: i32,
}

impl IndependentSampler {
    pub fn new(sample_count: i32) -> IndependentSampler {
        IndependentSampler {
            base_seed: 123,
            sample_count,
            current_sample: 0,
            current_dimension: 0,
        }
    }
}

impl Sampler for IndependentSampler {
    fn start_pixel(&mut self, _x: i32, _y: i32) {}

    fn advance(&mut self) {
        self.current_dimension = 0;
        self.current_sample += 1;
    }

    fn next1f(&mut self, rng: &mut impl Rng) -> f32 {
        self.current_dimension += 1;
        rng.gen()
    }

    fn next2f(&mut self, rng: &mut impl Rng) -> Vec2 {
        self.current_dimension += 2;
        Vec2::new(rng.gen(), rng.gen())
    }

    fn sample_count(&self) -> i32 {
        self.sample_count
    }
    fn seed(&self) -> u64 {
        self.base_seed
    }
    // fn set_rng(&mut self, rng: ChaCha8Rng) {
    //     self.rng = rng
    // }
}
