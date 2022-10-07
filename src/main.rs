
mod ray;
mod utils;
mod image2d;
mod surface;
mod camera;
mod transform;
mod scene;

// use std::rc::Rc;
// use std::ops::{Index, IndexMut};
// use nalgebra::{Vector2, Vector3, Vector4, Matrix4};
// use nalgebra_glm::sqrt;
// use serde::Deserializer;
// use serde_json::{Result, Value, json};
// use std::cmp;
// use assert_approx_eq::assert_approx_eq;
// // use indicatif::ProgressBar;
// use std::{cmp::min, fmt::Write};
// use indicatif::{ProgressBar, ProgressState, ProgressStyle};
// use atomic_counter::RelaxedCounter;


// use rustrt::ray::Ray;
// use rustrt::transform::Transform;
// use rustrt::utils::{rad2deg, luminance, lerp};
// use rustrt::image2d::Image2d;
// use rustrt::surface::{Sphere,Lambertian, Metal, Surface, Material, HitInfo, create_material};
// use rustrt::surface::SurfaceGroup;

use rustrt::scene::Scene;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The filename of the JSON scenefile to load (or the string \"example_sceneN\", where N is 0, 1, 2, or 3).
    #[arg(short, long)]
    scene: std::path::PathBuf,

    /// Specify just the output image format; default: png
    #[arg(short, long, default_value_t=String::new())]
    format: String,
    
    /// Specify the output image filename (extension must be one accepted by -f)
    #[arg(short, long)]
    outfile: std::path::PathBuf,

    // /// The path to the file to read
    // verbosity: i32,

    // /// Seed for the random number generator
    // #[arg(short, long, default_value_t = 1)]
    // seed: i32

}

fn main() {
    let args = Cli::parse();
}
