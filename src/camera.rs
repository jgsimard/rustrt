use nalgebra::{Vector2, Vector3, Matrix4, Matrix3x4};


use serde::{Serialize, Deserialize};
use serde_json::{Result, Value, json, from_value};


use crate::ray::Ray;
use crate::utils::{random_in_unit_disk, deg2rad};
use crate::transform::{Transform, parse_transform};


pub struct Camera{
    origin: Vector3<f32>,
    lower_left_corner: Vector3<f32>,
    horizontal: Vector3<f32>,
    vertical: Vector3<f32>,
    cu: Vector3<f32>,
    cv: Vector3<f32>,
    lens_radius: f32
}

impl Camera{
    pub fn new(
        lookfrom: Vector3<f32>,
        lookat: Vector3<f32>,
        vup: Vector3<f32>,
        vfov: f32, 
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32
    ) -> Camera {

        const FOCAL_LENGHT: f32 = 1.0;

        // Vertical field-of-view in degrees
        let theta = std::f32::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        
        let cw = (lookfrom - lookat).normalize();
        let cu = vup.cross(&cw).normalize();
        let cv = cw.cross(&cu);

        let h = focus_dist * viewport_width * cu;
        let v = focus_dist * viewport_height * cv;

        let llc = lookfrom - h / 2.0 - v / 2.0 - focus_dist * cw;

        Camera{
            origin: lookfrom,
            lower_left_corner: llc,
            horizontal: h,
            vertical: v,
            cu: cu,
            cv: cv,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = self.lens_radius * random_in_unit_disk(&mut rng);
        let offset = self.cu * rd.x + self.cv * rd.y;

        Ray::new(
            self.origin + offset, 
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset)
    }
}


// A virtual (pinhole) camera.
//
// The camera is responsible for generating primary rays. It is positioned
// using a Transform and points along the -z axis of the local coordinate
// system. It has an image plane positioned a z = -dist with size
// (width, height).
//
// We currently only support pinhole perspective cameras. This class could
// be made into a virtual base class to support other types of cameras
// (e.g. an orthographic camera, or omni-directional camera).
//
// The camera setup looks something like this, where the
// up vector points out of the screen:
//
// \verbatim
//         top view                         side view
//            ^                    up
//            |                     ^
//            |                     |             _,-'
//          width                   |         _,-'   |
//       +----|----+     +          |     _,-'       | h
//        \   |   /    d |        e | _,-'           | e
//         \  |  /     i |        y +'---------------+-i----->
//          \ | /      s |        e  '-,_   dist     | g
//           \|/       t |               '-,_        | h
//            +          +                   '-,_    | t
//           eye                                 '-,_|
// \endverbatim

 

pub struct PinholeCamera{
    pub transform: Transform,       // Local coordinate system
    pub size: Vector2<f32>,         // Physical size of the image plane
    pub focal_distance: f32,                 // Focal distance : Distance to image plane along local z axis
    pub resolution: Vector2<f32>,   // Image resolution
    pub aperture_radius: f32               // The size of the aperture for depth of field
}





// #[derive(Serialize, Deserialize, Debug)]
// struct CameraJson{
//     radius: f32,

//     #[serde(default = "default_center")]
//     center: Vector3<f32>
// }
// fn default_center() -> Vector3<f32>{
//     Vector3::zeros()
// }

impl PinholeCamera {
    pub fn new(json: Value) -> PinholeCamera{
        let resolution: Vector2<f32> = from_value(json["resolution"].clone()).unwrap_or_else(|x|Vector2::new(512., 512.));
        let aperture_radius: f32 = from_value(json["aperture"].clone()).unwrap_or_else(|x| 0.);
        let focal_distance: f32 = from_value(json["fdist"].clone()).unwrap_or_else(|x| 1.);
        let vfov: f32 = from_value(json["vfov"].clone()).unwrap_or_else(|x| 90.);
        let transform: Transform = parse_transform(&json);

        // Assignment 1: read the vertical field-of-view from j ("vfov"),
        // and compute the width and height of the image plane. Remember that
        // the "vfov" parameter is specified in degrees, but C++ math functions
        // expect it in radians. You can use deg2rad() from common.h to convert
        // from one to the other
        let height = 2.0 * focal_distance * deg2rad(vfov / 2.0).tan();
        let width = resolution[0] / resolution[1] * height;
        let size = Vector2::new(width,  -height); // FIXME : might need to change this


        PinholeCamera { 
            transform: transform, 
            size: size, 
            focal_distance: focal_distance, 
            resolution: resolution, 
            aperture_radius: 
            aperture_radius 
        }
    }

    pub fn generate_ray(&self, pixel: &Vector2<f32>) -> Ray {
        let origin = Vector3::zeros();
        let xy = self.size.component_mul(&pixel).component_div(&self.resolution) - self.size / 2.0;
        let direction = Vector3::new(xy.x, xy.y, -self.focal_distance);
        self.transform.ray(&Ray::new(origin, direction))
    }
    
}