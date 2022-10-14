mod aabb;
mod camera;
mod image2d;
mod materials;
mod ray;
mod scene;
mod surfaces;
mod transform;
mod utils;

mod example_scenes;

use crate::example_scenes::create_example_scene;
use crate::scene::Scene;

use clap::Parser;
use serde_json::Value;
use std::path::PathBuf;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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
    outfile: PathBuf,
    // /// The path to the file to read
    // verbosity: i32,

    // /// Seed for the random number generator
    // #[arg(short, long, default_value_t = 1)]
    // seed: i32
}

fn read_scene_from_file<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let j = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(j)
}

fn main() {
    let args = Cli::parse();

    println!("scene : {}", args.scene.to_string_lossy());

    let scene_json = if args.scene.exists() {
        println!("existing file");
        read_scene_from_file(args.scene).unwrap()
    } else if args.scene.to_string_lossy().parse::<i32>().is_ok() {
        let index = args.scene.to_string_lossy().parse::<i32>().unwrap();
        create_example_scene(index)
    } else {
        panic!("I dont know how to parse {}", args.scene.to_string_lossy());
    };
    // println!("{}", scene_json);

    let scene = Scene::new(scene_json);

    // let outfile = "something".to_string();

    let image = scene.raytrace();

    // println!("Number of intersection tests: {}", intersection_tests);
    // println!("Number of rays traced: {}", rays_traced);
    // println!("Average number of intersection tests per ray: {}", (intersection_tests as f32) / (rays_traced as f32));
    // println!("Writing rendered image to file \"{}\"...", outfile);

    image.save(args.outfile.to_str().unwrap().to_string());

    println!("Done");
}
