use nalgebra::{Vector2, Vector3};

use serde_json::{from_value, Value};

use crate::ray::Ray;
use crate::transform::{parse_transform, Transform};
use crate::utils::deg2rad;

/// A virtual (pinhole) camera.
///
/// The camera is responsible for generating primary rays. It is positioned
/// using a Transform and points along the -z axis of the local coordinate
/// system. It has an image plane positioned a z = -dist with size
/// (width, height).
///
/// We currently only support pinhole perspective cameras. This class could
/// be made into a virtual base class to support other types of cameras
/// (e.g. an orthographic camera, or omni-directional camera).
///
/// The camera setup looks something like this, where the
/// up vector points out of the screen:
///
/// ```text
///         top view                         side view
///            ^                    up
///            |                     ^
///            |                     |             _,-'
///          width                   |         _,-'   |
///       +----|----+     +          |     _,-'       | h
///        \   |   /    d |        e | _,-'           | e
///         \  |  /     i |        y +'---------------+-i----->
///          \ | /      s |        e  '-,_   dist     | g
///           \|/       t |               '-,_        | h
///            +          +                   '-,_    | t
///           eye                                 '-,_|
/// ```

#[derive(Debug)]
pub struct PinholeCamera {
    /// Local coordinate system
    pub transform: Transform,     
    /// Physical size of the image plane
    pub size: Vector2<f32>,       
    /// Distance to image plane along local z axis
    pub focal_distance: f32,      
    /// Image resolution
    pub resolution: Vector2<f32>, 
    /// The size of the aperture for depth of field
    pub aperture_radius: f32,     
}


impl PinholeCamera {
    pub fn new(json: &Value) -> PinholeCamera {
        let resolution: Vector2<f32> =
            from_value(json["resolution"].clone()).unwrap_or(Vector2::new(512., 512.));
        let aperture_radius: f32 = from_value(json["aperture"].clone()).unwrap_or(0.);
        let focal_distance: f32 = from_value(json["fdist"].clone()).unwrap_or(1.);
        let vfov: f32 = from_value(json["vfov"].clone()).unwrap_or(90.);
        let transform: Transform = parse_transform(&json);

        // Assignment 1: read the vertical field-of-view from j ("vfov"),
        // and compute the width and height of the image plane. Remember that
        // the "vfov" parameter is specified in degrees, but C++ math functions
        // expect it in radians. You can use deg2rad() from common.h to convert
        // from one to the other
        let height = 2.0 * focal_distance * deg2rad(vfov / 2.0).tan();
        let width = resolution[0] / resolution[1] * height;
        let size = Vector2::new(width, -height); // FIXME : might need to change this

        PinholeCamera {
            transform: transform,
            size: size,
            focal_distance: focal_distance,
            resolution: resolution,
            aperture_radius: aperture_radius,
        }
    }

    /// Generate a ray inside a given pixel
    pub fn generate_ray(&self, pixel: &Vector2<f32>) -> Ray {
        let origin = Vector3::zeros();
        let xy = self
            .size
            .component_mul(&pixel)
            .component_div(&self.resolution)
            - self.size / 2.0;
        let direction = Vector3::new(xy.x, xy.y, -self.focal_distance);
        self.transform.ray(&Ray::new(origin, direction))
    }
}
