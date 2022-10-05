#![allow(unused)]


use serde::Deserialize;
mod ray;
mod utils;
mod image2d;
// mod hit;
// mod sphere;
mod camera;
// mod material;
mod transform;
// mod scene;


use std::ops::{Index, IndexMut};
use nalgebra::{Vector2, Vector3, Vector4, Matrix4};
use serde::Deserializer;
use serde_json::{Result, Value, json};
use std::cmp;
use assert_approx_eq::assert_approx_eq;


use crate::ray::Ray;
use crate::transform::Transform;
use crate::utils::{rad2deg, luminance, lerp};
use crate::image2d::Image2d;
// use hit::{Hit, World};
// use sphere::Sphere;
use camera::{Camera, PinholeCamera};
// use material::{Lambertian, Metal, Dielectric};


fn vec2color(dir: &Vector3<f32>) -> Vector3<f32>{
    0.5 * (dir.add_scalar(1.0))
}

fn ray2color(ray: &Ray) -> Vector3<f32>{
    return vec2color(&ray.direction.normalize())
}


// Generate rays by hand
fn test_manual_camera_image()
{
    println!("");
    println!("{}{}{}", 
    "--------------------------------------------------------\n",
    "PROGRAMMING ASSIGNMENT, PART 1: Generating rays by hand \n",
    "--------------------------------------------------------\n");

    // Setup the output image
    let mut ray_image = Image2d::new(200, 100);

    let camera_origin = Vector3::new(0., 0., 0.);
    let image_plane_width  = 4.;
    let image_plane_height = 2.;

    // loop over all pixels and generate a ray
    for y in 0..ray_image.size_y
    {
        for x in 0..ray_image.size_x
        {
            // Fill in ray_origin so that the ray starts at
            // camera_origin, and fill in ray_direction so that
            // 1) the x component of the direction varies from -image_plane_width/2 for the left-most pixel to
            //    +image_plane_width/2 for the right-most pixel
            // 2) the y component of the direction varies from +image_plane_height/2 for the top-most pixel to
            //    -image_plane_height/2 for the bottom-most pixel
            // 3) the z component is -1
            // 
            // Make sure to calculate the ray directions to go through the center of each pixel
            let ray_origin = camera_origin.clone();
            let x_ =  lerp(-image_plane_width/2.0 , image_plane_width / 2.0 , (x as f32) / (ray_image.size_x as f32));
            let y_ =  lerp(image_plane_height/2.0, -image_plane_height / 2.0, (y as f32) / (ray_image.size_y as f32));
            let ray_direction = Vector3::new(x_, y_, -1.0) - ray_origin;
            let ray = Ray::new(ray_origin, ray_direction);

            // Generate a visual color for the ray so we can debug our ray directions
            ray_image[(x, y)] = ray2color(&ray);
        }
    }

    let filename = "scenes/assignment1/01_manual_ray_image.png".to_string();
    println!("Saving ray image to {}....", filename);
    ray_image.save(filename);
}


fn test_json()
{
    // Darts also includes a C++ library (https://github.com/nlohmann/json)
    // for parsing and manipulating JSON data.
    //
    // JSON is a human-readible data interchange format for expressing
    // attribute-value pairs. You can read more about it here:
    //      https://en.wikipedia.org/wiki/JSON
    //      https://www.json.org/
    //
    // In darts, we will use it for two purposes:
    //  1) As a generic way to pass named parameters to functions
    //  2) As a way to specify and load text-based scene files

    println!("");
    println!("{}{}{}", 
    "--------------------------------------------------------\n",
    "PROGRAMMING ASSIGNMENT, PART 2: passing data using JSON \n",
    "--------------------------------------------------------\n");

    let f = 2.0;
    let s = "a text string".to_string();
    let c3f = Vector3::new(1.0, 0.25, 0.5);
    let v3f = Vector3::new(2.0, 3.0, 4.0);
    let v4f = Vector4::new(2.0, 3.0, 4.0, 5.0);
    let n3f = Vector3::new(2.0, 3.0, 4.0);
    println!("Original darts data:\nf = {},\ns = {},\nc3f = {},\nv3f = {},\nv4f = {},\nn3f = {}.", f, s, c3f, v3f, v4f, n3f);

    // All the basic darts data-types can easily be stored in a JSON object
    let j = json!({
        "my float": f,
        "my string": s,
        "my color" : c3f,
        "my vector3": v3f,
        "my normal": n3f
    });

    println!("The JSON object contains: {}", j.to_string());

    // serde_json::fr
    // We can also read these structures back out of the JSON object
    let  f2 = &j["my float"];
    let s2 = &j["my string"];
    let c3f2: Vector3<f32> = serde_json::from_value(j["my color"].clone()).expect("could not read 'my color'");
    let v3f2: Vector3<f32> = serde_json::from_value(j["my vector3"].clone()).expect("could not read 'my vector3'");
    let n3f2: Vector3<f32> = serde_json::from_value(j["my normal"].clone()).expect("could not read 'my normal'");

    println!("Retrieved darts data:\nf2 = {},\ns2 = {},\nc3f2 = {},\nv3f2 = {},\nn3f2 = {}.",
                 f2, s2, c3f2, v3f2, n3f2);
    // There is a bug in the code above, and c3f2 doesn't have the same
    // value as the original c3f. Fix it.

    // Now we will pass a json object in place of explicit parameters to
    // a function. Go to the function below and implement the TODO.
    let parameters = json!({"radius": 2.3});
    function_with_json_parameters(&parameters);
}

fn function_with_json_parameters(j: &Value)
{
    // Many of the constructors for ray tracing in darts take a JSON object. This
    // allows us to have a uniform interface for creating these structures while
    // allowing the constructors to retrieve the necessary values from the JSON
    // object. This will simplify our code for writing a parser for reading
    // scene files from disk.

    // Sometimes we may want to make a parameter optional, and take on some
    // default value if it is not specified.
    // Unfortunately, checking for a missing parameter using e.g. j["radius"]
    // will throw an exception if the parameter doesn't exist (if j is const).
    // Instead, we can use j.value<type>("name", default_value) to extract it.
    // This is what the constructors to Camera, Sphere, Quad, and Materials do.

    // Replace the below two lines to extract the parameters radius (default=1.f),
    // and center (default={0,0,0}) from the JSON object j
    
    let radius: f32 = serde_json::from_value(j["radius"].clone()).unwrap_or_else(|x|1.0);
    let center: Vector3<f32> = serde_json::from_value(j["center"].clone()).unwrap_or_else(|x|Vector3::zeros());
    println!("The radius is: {}", radius);
    println!("The center is:\n{}", center);
}

use std::collections::HashMap;

// Next, we will generate the same image, but using the Camera class
fn test_camera_class_image()
{
    println!("");
    println!("{}{}{}",
    "--------------------------------------------------------\n",
    "PROGRAMMING ASSIGNMENT, PART 3: Camera class generate_ray\n",
    "--------------------------------------------------------\n");

    // Setup the output image
    let mut ray_image = Image2d::new(200, 100);

    // Set up a camera with some reasonable parameters, using JSON
    // Look in camera.h and implement the camera constructor

    let camera_data = r#"{
        "vfov": 90.0,
        "resolution": [200, 100],
        "fdist": 1.0
    }"#;

    let camera_map: Value = serde_json::from_str(camera_data).unwrap();
 
    let camera: PinholeCamera = PinholeCamera::new(camera_map);

    // println!("{}, {}, {}, {}, {}", camera.size, camera.focal_distance, camera.resolution, camera.aperture_radius, camera.transform.m);

    // loop over all pixels and ask the camera to generate a ray
    for y in 0..ray_image.size_y
    {
        for x in 0..ray_image.size_x
        {
            // Look in camera.h|cpp and implement Camera::generate_ray

            // We add 0.5 to the pixel coordinate to center the ray within the pixel
            let ray        = camera.generate_ray(&Vector2::new((x as f32) + 0.5, (y as f32) + 0.5));
            ray_image[(x, y)] = ray2color(&ray);
        }
    }

    let filename = "scenes/assignment1/01_camera_ray_image.png".to_string();
    println!("Saving ray image to {}....", filename);
    ray_image.save(filename);
}

fn test_transforms()
{
    println!("");
    println!("{}{}{}",
    "--------------------------------------------------------\n",
    "PROGRAMMING ASSIGNMENT, PART4: Transforms              \n",
    "--------------------------------------------------------\n");

    // Darts also provides you with a Transform class.
    // Transform is a helper class that helps you transform geometric primitives
    // correctly Internally, it keeps track of a transformation matrix and its
    // inverse

    // Let's create a random transformation matrix
    let transformation_matrix = Matrix4::new(
        -0.846852, 0.107965, -0.520755, 0.0, 
        -0.492958, -0.526819, 0.692427, 0.0,
        -0.199586, 0.843093, 0.499359, 0.0, 
        -0.997497, 0.127171, -0.613392, 1.0).transpose();

    // Now that we have a matrix, we can create a transform from it:
    let transform = Transform::new(transformation_matrix);

    // Go to transform.h and implement all required methods there. If you
    // implement them correctly, the code below will work:

    // Let's create some random geometric objects...

    let vector = Vector3::new(-0.997497, 0.127171, -0.6133920);
    let point = Vector3::new(0.617481, 0.170019, -0.0402539);
    let normal = Vector3::new(-0.281208, 0.743764, 0.6064130);
    let ray = Ray::new(
        Vector3::new(-0.997497, 0.127171, -0.613392), 
        Vector3::new(0.962222, 0.264941, -0.0627278));

    println!("vector = {}.", vector);
    println!("point  = {}.", point);
    println!("normal = {}.", normal);
    println!("ray.o  = {};\nray.d  = {}.", ray.origin, ray.direction);

    // ...and let's transform them!
    // We can transform things simply by multiplying it with the transform.
    // Let's check if you did it correctly:
    let transformed_vector = transform.vector(&vector);
    let transformed_point  = transform.point(&point);
    let transformed_normal = transform.normal(&normal);
    let transformed_ray    = transform.ray(&ray);

    let correct_transformed_vector= Vector3::new(0.904467, -0.6918370, 0.301205);
    let correct_transformed_point= Vector3::new(-1.596190, 0.0703303, -0.837324);
    let correct_transformed_normal= Vector3::new(-0.249534, 0.0890737, 0.96426);
    let correct_transformed_ray_position= Vector3::new(-0.0930302, -0.564666, -0.312187);
    let correct_transformed_ray_direction= Vector3::new(-0.932945, -0.088575, -0.348953);

    let vector_error = (correct_transformed_vector - transformed_vector).abs().max();
    let point_error  = (correct_transformed_point - transformed_point).abs().max();
    let normal_error = (correct_transformed_normal - transformed_normal).abs().max();
    let ray_error    = (correct_transformed_ray_position - transformed_ray.origin).abs().max().max(
        (correct_transformed_ray_direction - transformed_ray.direction).abs().max());

    println!("The forward transform matrix is\n{}.", transform.m);
    println!("The inverse transform matrix is\n{}.", transform.m_inv);

    println!("Result of transform*vector is:\n{}, and it should be:\n{}.", transformed_vector, correct_transformed_vector);
    assert!(vector_error < 1e-5);
    
    println!("Result of transform*point is:\n{}, and it should be:\n{}.", transformed_point, correct_transformed_point);
    assert!(point_error < 1e-5);

    println!("Result of transform*normal is:\n{}, and it should be:\n{}.", transformed_normal, correct_transformed_normal);
    assert!(normal_error < 1e-5);

    println!("transform*ray: transformed_ray.o is:\n{}, and it should be:\n{}.", transformed_ray.origin, correct_transformed_ray_position);
    println!("transform*ray: transformed_ray.d is:\n{}, and it should be:\n{}.", transformed_ray.direction, correct_transformed_ray_direction);    
    assert!(ray_error < 1e-5);

}
fn main() {
    // test_vector_and_matrices();
    // test_manual_camera_image();
    // test_json();
    // test_camera_class_image();
    test_transforms()
}
