mod independent;

use enum_dispatch::enum_dispatch;
use nalgebra_glm::Vec2;
use rand::Rng;
use serde_json::{json, Map, Value};

use crate::core::utils::read_or;

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
    fn next1f(&mut self, rng: &mut impl Rng) -> f32;

    /// Retrieve the next two float values (dimensions) from the current sample
    fn next2f(&mut self, rng: &mut impl Rng) -> Vec2;

    /// Return the number of configured pixel samples
    fn sample_count(&self) -> i32;

    fn seed(&self) -> u64;
}

use crate::samplers::independent::IndependentSampler;

#[enum_dispatch(Sampler)]
#[derive(Debug, Clone)]
pub enum SamplerType {
    Independent(IndependentSampler),
}

pub fn create_sampler(map: &Map<String, Value>) -> SamplerType {
    // TODO : rewrite this
    let sampler_json = if let Some(sampler_value) = map.get("sampler") {
        let mut sampler_map = sampler_value.as_object().unwrap().clone();
        if !sampler_map.contains_key("type") {
            println!("No sampler 'type' specified, assuming independent sampling.");
            sampler_map.insert("type".to_string(), json!("independent"));
        }
        if !sampler_map.contains_key("samples") {
            println!("Number of samples is not specified, assuming 1.");
            sampler_map.insert("samples".to_string(), json!(1));
        }
        serde_json::to_value(sampler_map).unwrap()
    } else {
        println!("No sampler specified, defaulting to 1 spp independent sampling.");
        json!({"type" : "independent", "samples": 1})
    };

    let sampler_type = sampler_json
        .get("type")
        .expect("no sampler type")
        .as_str()
        .expect("lolz");

    match sampler_type {
        "independent" => {
            let samples = read_or(&sampler_json, "samples", 1);
            SamplerType::Independent(IndependentSampler::new(samples))
        }
        _ => unimplemented!("Sampler type {}", sampler_type),
    }
}
