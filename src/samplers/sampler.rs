extern crate nalgebra_glm as glm;

use crate::utils::read_or;
use enum_dispatch::enum_dispatch;
use glm::Vec2;
use rand::Rng;

use serde_json::Value;

/// Sample generator.
///
/// A sample generator is responsible for generating the random number stream that will be passed to an #Integrator
/// implementation as it computes the radiance incident along a specified ray.
#[enum_dispatch]
pub trait Sampler {
    // /// Deterministically seed the underlying RNG (to produce identical results between runs)
    // fn seed(&mut self, seed: u64);

    ///Prepare to generate samples for pixel (x,y).
    ///
    /// This function is called every time the integrator starts rendering a new pixel.
    fn start_pixel(&mut self, x: i32, y: i32);

    /// Advance to the next sample
    fn advance(&mut self);

    /// Retrieve the next float value (dimension) from the current sample
    fn next1f(&self, rng: &mut impl Rng) -> f32;

    /// Retrieve the next two float values (dimensions) from the current sample
    fn next2f(&self, rng: &mut impl Rng) -> Vec2;

    /// Return the number of configured pixel samples
    fn sample_count(&self) -> i32;

    fn seed(&self) -> u64;

    // fn set_rng(&mut self, rng: ChaCha8Rng);
}

#[enum_dispatch(Sampler)]
#[derive(Debug, Clone)]
pub enum SamplerType {
    IndependentSampler,
}

/// Independent sampling
///
/// returns independent uniformly distributed random numbers on \f$[0, 1)^2\f$
#[derive(Debug, Clone)]
pub struct IndependentSampler {
    pub base_seed: u64,
    pub sample_count: i32,
    pub current_sample: i32,
    pub current_dimension: i32,
    // pub rng: ChaCha8Rng,
}

impl Sampler for IndependentSampler {
    fn start_pixel(&mut self, _x: i32, _y: i32) {}

    fn next1f(&self, rng: &mut impl Rng) -> f32 {
        // self.current_dimension += 1;
        rng.gen()
    }

    fn next2f(&self, rng: &mut impl Rng) -> Vec2 {
        // self.current_dimension += 2;
        Vec2::new(rng.gen(), rng.gen())
    }

    fn advance(&mut self) {
        self.current_dimension = 0;
        self.current_sample += 1;
    }

    fn seed(&self) -> u64 {
        self.base_seed
    }
    fn sample_count(&self) -> i32 {
        self.sample_count
    }
    // fn set_rng(&mut self, rng: ChaCha8Rng) {
    //     self.rng = rng
    // }
}

pub fn create_sampler(j: &Value) -> SamplerType {
    let sampler_type = j
        .get("type")
        .expect("no sampler type")
        .as_str()
        .expect("lolz");

    match sampler_type {
        "independent" => {
            // ThreadRng::from(_)
            let samples = read_or(j, "samples", 1);
            SamplerType::from(IndependentSampler {
                base_seed: 123,
                sample_count: samples,
                current_sample: 0,
                current_dimension: 0,
                // rng: ChaCha8Rng::seed_from_u64(123),
            })
        }
        _ => {
            unimplemented!("Sampler type {}", sampler_type);
        }
    }
}
