#![allow(unused)]

use clap::Parser;

mod ray;
mod hit;
mod sphere;
mod camera;
mod material;
mod utils;
mod transform;
mod scene;

use std::io::{stderr, Write};
use std::rc::Rc;
use rand::Rng;
use nalgebra::Vector3;
// use nalgebra::Mul

use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::{Camera, PinholeCamera};
use material::{Lambertian, Metal, Dielectric};


// /// Search for a pattern in a file and display the lines that contain it.
// #[derive(Parser)]
// struct Cli {
//     /// The pattern to look for
//     pattern: String,
//     /// The path to the file to read
//     path: std::path::PathBuf,
// }

// fn main() {
    // let args = Cli::parse();
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
// }


use serde_json::{Result, Value};


// pub struct Image(Vec<Vec<Vector3<f32>>>);

// impl Image {
//     pub fn compute(nx: usize, ny: usize, mut f: impl FnMut(usize, usize) -> Vector3<f32>) -> Image{
//         Image(
//             (0..ny)
//             .rev()
//             .map(|y| (0..nx).map(|x| f(x,y)).collect()).collect()
//         )
//     }
// }
// pub struct SurfaceGroup{
//     a: f32
// }


// pub struct Scene{
//     camera: PinholeCamera,
//     surfaces: SurfaceGroup,
//     background: Vector3<f32>,
//     num_samples: i32
// }

// impl Scene{
//     pub fn parse(json: Value) -> Scene{

//     }

//     pub fn recursive_color(ray: &Ray, depth: i32) -> Vector3<f32>{

//     }   
    
//     pub fn raytrace() -> Image {

//     }
// }

fn main() {
    let data = r#"
    {
        "camera": {
            "transform": {
                "o": [0, 0, 4]
            },
            "vfov": 45,
            "resolution": [640, 480]
        },
        "sampler": {
            "type": "independent",
            "samples": 100
        },
        "background": [
            1, 1, 1
        ],
        "surfaces": [
            {
                "type": "sphere",
                "radius": 1,
                "material": {
                    "type": "lambertian",
                    "albedo": [0.6, 0.4, 0.4]
                }
            }, {
                "type": "quad",
                "transform": {
                    "o": [
                        0, -1, 0
                    ],
                    "x": [
                        1, 0, 0
                    ],
                    "y": [
                        0, 0, -1
                    ],
                    "z": [0, 1, 0]
                },
                "size": 100,
                "material": {
                    "type": "lambertian",
                    "albedo": [0.75, 0.75, 0.75]
                }
            }
        ]
    }"#;

    let v = Vector3::new(0.1, 0.2, 0.3);
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    println!("{}", v["camera"])

}
