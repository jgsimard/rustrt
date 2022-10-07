
mod ray;
mod utils;
mod image2d;
mod surface;
mod camera;
mod transform;
mod scene;

use rustrt::scene::Scene;
use clap::Parser;
use serde_json::json;

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

    let mock_json = json!({"patate": 1});

    let scene = Scene::new(mock_json);

    let outfile = "something".to_string();

    let image = scene.raytrace();

    println!("Number of intersection tests: {}", intersection_tests);
    println!("Number of rays traced: {}", rays_traced);
    println!("Average number of intersection tests per ray: {}", (intersection_tests as f32) / (rays_traced as f32));
    println!("Writing rendered image to file \"{}\"...", outfile);

    image.save(outfile);

    println!("Done");
}
