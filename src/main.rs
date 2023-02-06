mod core;
mod integrators;
mod materials;
mod samplers;
mod surfaces;
mod textures;

#[cfg(test)]
mod tests;

mod example_scenes;

use crate::core::scene::Scene;
use crate::example_scenes::create_example_scene;

use clap::Parser;
use serde_json::Value;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Parser)]
struct Cli {
    /// The filename of the JSON scenefile to load (or the string \"example_sceneN\", where N is 0, 1, 2, or 3).
    #[arg(short, long, default_value_t=String::from("3"))]
    scene: String,

    /// Specify just the output image format; default: png
    #[arg(short, long, default_value_t=String::from("png"))]
    format: String,

    /// Specify the output image filename (extension must be one accepted by -f)
    #[arg(short, long, default_value_t=String::from("test.png"))]
    outfile: String,
}

fn read_scene_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file
    let j = serde_json::from_reader(reader)?;

    Ok(j)
}

use crate::core::utils::INTERSECTION_TEST;
use crate::core::utils::RAYS;
use std::sync::atomic::Ordering;

fn main() {
    let args = Cli::parse();

    println!("scene : {:?}", args.scene);

    let path = PathBuf::from(args.scene.clone());
    let scene_json = if path.exists() {
        println!("scene existing file");
        read_scene_from_file(path).unwrap()
    } else if args.scene.parse::<i32>().is_ok() {
        let index = args.scene.parse::<i32>().unwrap();
        create_example_scene(index)
    } else {
        panic!("I dont know how to parse {:?}", args.scene);
    };

    let scene = Scene::new(&scene_json);
    let image = scene.raytrace();

    println!("Number of intersection tests: {INTERSECTION_TEST:?}");
    println!("Number of rays traced: {RAYS:?}");
    println!(
        "Average number of intersection tests per ray: {}",
        (INTERSECTION_TEST.load(Ordering::SeqCst) as f32) / (RAYS.load(Ordering::SeqCst) as f32)
    );
    println!("Writing rendered image to file {:?}", args.outfile);

    image.save(&PathBuf::from(args.outfile));

    println!("Done");
}
