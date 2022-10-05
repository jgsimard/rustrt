mod ray;
mod hit;
mod sphere;
mod camera;
mod material;
mod utils;

use std::io::{stderr, Write};
use std::rc::Rc;
use rand::Rng;
use nalgebra::Vector3;

use ray::Ray;
use hit::{Hit, World};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};


fn ray_color(r: &Ray, world: &World, depth: u32) -> Vector3<f32> {
    if depth <= 0{
        // If we've exceeded the ray bounce limit, no more light is gathered
        return Vector3::<f32>::zeros();
    }
    // let mut rng = rand::thread_rng();
    if let Some(rec) = world.hit(r, 0.0001, f32::INFINITY){
        if let Some((attenuation, scattered)) = rec.material.scatter(&r, &rec){
            // attenuation * ray_color(&scattered, world, depth -1)
            // attenuation.zip_map(&ray_color(&scattered, world, depth -1), |l,r| l*r)
            attenuation.component_mul(&ray_color(&scattered, world, depth -1))
        } else{
            Vector3::<f32>::zeros()
        }
    } else{
        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t)* Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn format_color(v: Vector3<f32>, sample_per_pixel: u32) -> String{
    format!("{} {} {}", 
    (255.999 * (v.x / (sample_per_pixel as f32)).sqrt().clamp(0.0, 0.999)) as u32,
    (255.999 * (v.y / (sample_per_pixel as f32)).sqrt().clamp(0.0, 0.999)) as u32, 
    (255.999 * (v.z / (sample_per_pixel as f32)).sqrt().clamp(0.0, 0.999)) as u32)
}

pub struct Image(Vec<Vec<Vector3<f32>>>);

impl Image {
    pub fn compute(nx: usize, ny: usize, mut f: impl FnMut(usize, usize) -> Vector3<f32>) -> Image{
        Image(
            (0..ny)
            .rev()
            .map(|y| (0..nx).map(|x| f(x,y)).collect()).collect()
        )
    }
}

// #[allow(unused)]
// fn two_balls_scene(nx: u32, ny: u32) -> (World, Camera){
//     // World
//     let mut world = World::new();
//     world.push(Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)));
//     world.push(Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)));

//     // Camera
//     let camera = Camera::new();

//     (world, camera)
// }

// #[allow(unused)]
// fn shiny_metal_scene(nx: u32, ny: u32) -> (World, Camera){
//         // World
//         let mut world = World::new();
//         let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//         let mat_center = Rc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
//         let mat_left = Rc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.0));
//         let mat_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0));
    
//         let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
//         let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center);
//         let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
//         let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    
//         world.push(Box::new(sphere_ground));
//         world.push(Box::new(sphere_center));
//         world.push(Box::new(sphere_left));
//         world.push(Box::new(sphere_right));
    
//         // Camera
//         let camera = Camera::new();

//     (world, camera)
// }

// #[allow(unused)]
// fn fuzzy_metal_scene(nx: u32, ny: u32) -> (World, Camera){
//         // World
//         let mut world = World::new();
//         let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//         let mat_center = Rc::new(Lambertian::new(Vector3::new(0.7, 0.3, 0.3)));
//         let mat_left = Rc::new(Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3));
//         let mat_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));
    
//         let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
//         let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center);
//         let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
//         let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    
//         world.push(Box::new(sphere_ground));
//         world.push(Box::new(sphere_center));
//         world.push(Box::new(sphere_left));
//         world.push(Box::new(sphere_right));
    
//         // Camera
//         let camera = Camera::new();

//     (world, camera)
// }

// #[allow(unused)]
// fn glass_metal_scene(nx: u32, ny: u32) -> (World, Camera){
//         // World
//         let mut world = World::new();
//         let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//         let mat_center = Rc::new(Dielectric::new(1.5));
//         let mat_left = Rc::new(Dielectric::new(1.5));
//         let mat_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));
    
//         let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
//         let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center);
//         let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
//         let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    
//         world.push(Box::new(sphere_ground));
//         world.push(Box::new(sphere_center));
//         world.push(Box::new(sphere_left));
//         world.push(Box::new(sphere_right));
    
//         // Camera
//         let camera = Camera::new();

//     (world, camera)
// }

// #[allow(unused)]
// fn glass_approx_metal_scene(nx: u32, ny: u32) -> (World, Camera){
//         // World
//         let mut world = World::new();
        
//         let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//         let mat_center = Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
//         let mat_left = Rc::new(Dielectric::new(1.5));
//         let mat_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));
    
//         let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
//         let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center);
//         let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
//         let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, mat_right);
    
//         world.push(Box::new(sphere_ground));
//         world.push(Box::new(sphere_center));
//         world.push(Box::new(sphere_left));
//         world.push(Box::new(sphere_right));
    
//         // Camera
//         let camera = Camera::new();

//     (world, camera)
// }

// #[allow(unused)]
// fn hollow_glass_scene(nx: u32, ny: u32) -> (World, Camera){
//         // World
//         let mut world = World::new();
        
//         let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//             let mat_center = Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
//             let mat_left = Rc::new(Dielectric::new(1.5));
//             let mat_left_inner = Rc::new(Dielectric::new(1.5));
//             let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//             let mat_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

//             let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
//             let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center);
//             let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
//             let sphere_left_inner = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.4, mat_left_inner);
//             let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//             let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, mat_right);

//             world.push(Box::new(sphere_ground));
//             world.push(Box::new(sphere_center));
//             world.push(Box::new(sphere_left));
//             world.push(Box::new(sphere_left_inner));
//             let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
//             world.push(Box::new(sphere_right));
    
//         // Camera
//         let camera = Camera::new();

//     (world, camera)
// }

// #[allow(unused)]
// fn two_balls_close_scene(nx: u32, ny: u32) -> (World, Camera){
//     // World
//     let r: f32 = (std::f32::consts::PI / 4.0).cos(); 
//     let mut world = World::new();

//     let mat_left = Rc::new(Lambertian::new(Vector3::new(0.0, 0.0, 1.0)));
//     let mat_right = Rc::new(Lambertian::new(Vector3::new(1.0, 0.0, 0.0)));

//     let sphere_left = Sphere::new(Vector3::new(-r, 0.0, -1.0), r, mat_left);
//     let sphere_right = Sphere::new(Vector3::new(r, 0.0, -1.0), r, mat_right);

//     world.push(Box::new(sphere_left));
//     world.push(Box::new(sphere_right));

//     // Camera
//     const ASPECT_RATIO: f32 = 16.0 / 9.0;
//     let camera = Camera::new(90.0, ASPECT_RATIO);


//     (world, camera)
// }

#[allow(unused)]
fn balls_far_scene(nx: u32, ny: u32) -> (World, Camera){
    // World
    let mut world = World::new();

    let mat_ground = Rc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Dielectric::new(1.5));
    let mat_left_inner = Rc::new(Dielectric::new(1.5));
    let mat_right = Rc::new(Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0));

    let sphere_ground = Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let sphere_center = Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    let sphere_left = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let sphere_left_inner = Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45, mat_left_inner);
    let sphere_right = Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    world.push(Box::new(sphere_ground));
    world.push(Box::new(sphere_center));
    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_left_inner));
    world.push(Box::new(sphere_right));

    // Camera
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    // let camera = Camera::new(Vector3::new(-2.0, 2.0, 1.0),
    //                     Vector3::new(0.0, 0.0, -1.0),
    //                     Vector3::new(0.0, 1.0, 0.0),
    //                     90.0,
    //                     ASPECT_RATIO);

    // Camera
    let lookfrom = Vector3::new(3.0, 3.0, 2.0);
    let lookat = Vector3::new(0.0, 0.0, -1.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).norm();
    let aperture = 2.0;

    let camera = Camera::new(lookfrom,
                        lookat,
                        vup,
                        20.0,
                        ASPECT_RATIO,
                        aperture,
                        dist_to_focus);

    (world, camera)
}


#[allow(unused)]
fn random_scene() -> (World, Camera) {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Rc::new(Lambertian::new(Vector3::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Vector3::new((a as f32) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f32) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Vector3::new(
                    rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0) * rng.gen_range(0.0..1.0)
                );
                let sphere_mat = Rc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Vector3::new(
                    rng.gen_range(0.4..1.0),
                    rng.gen_range(0.4..1.0),
                    rng.gen_range(0.4..1.0)
                );
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Rc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    let mat2 = Rc::new(Lambertian::new(Vector3::new(0.4, 0.2, 0.1)));
    let mat3 = Rc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    // Image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    

    // Camera
    let lookfrom = Vector3::new(13.0, 2.0, 3.0);
    let lookat = Vector3::new(0.0, 0.0, 0.0);
    let vup = Vector3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom,
                          lookat,
                          vup,
                          20.0,
                          ASPECT_RATIO,
                          aperture,
                          dist_to_focus);

    (world, camera)
}
fn main() {
    // Image
    // const ASPECT_RATIO: f32 = 16.0 / 9.0;
    // const IMAGE_WIDTH: u32 = 256;
    // const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    // const SAMPLE_PER_PIXEL: u32 = 10;
    // const MAX_DEPTH: u32 = 5;

    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: u32 = 512;
    const IMAGE_HEIGHT: u32 = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as u32;
    const SAMPLE_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 10;


    // let (world, camera) = two_balls_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = shiny_metal_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = fuzzy_metal_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = glass_metal_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = glass_approx_metal_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = hollow_glass_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = two_balls_close_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    // let (world, camera) = balls_far_scene(IMAGE_WIDTH, IMAGE_HEIGHT);
    let (world, camera) = random_scene();


    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for y in (0..IMAGE_HEIGHT).rev(){
        eprint!("\rScanlines remaining : {:3}", y);
        stderr().flush().unwrap();
        for x in 0..IMAGE_WIDTH{
            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLE_PER_PIXEL{
                let mut rng = rand::thread_rng();

                let u = ((x as f32) + rng.gen::<f32>())/ (IMAGE_WIDTH as f32);
                let v = ((y as f32) + rng.gen::<f32>()) / (IMAGE_HEIGHT as f32);

                let r  = camera.get_ray(u, v);

                 pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            println!("{}", format_color(pixel_color, SAMPLE_PER_PIXEL));
        }
    }
    eprintln!("\nDone");
}
