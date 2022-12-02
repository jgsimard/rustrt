use nalgebra_glm::{Vec2, Vec3};
use serde_json::Value;

use crate::core::ray::Ray;
use crate::core::transform::Transform;
use crate::core::utils::{deg2rad, read_or};

/// A virtual pinhole camera.
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
    pub size: Vec2,
    /// Distance to image plane along local z axis
    pub focal_distance: f32,
    /// Image resolution
    pub resolution: Vec2,
    /// The size of the aperture for depth of field
    pub aperture_radius: f32,
}

impl PinholeCamera {
    pub fn new(json: &Value) -> PinholeCamera {
        let resolution = read_or(json, "resolution", Vec2::new(512., 512.));
        let aperture_radius = read_or(json, "aperture", 0.);
        let focal_distance = read_or(json, "fdist", 1.);
        let vfov = read_or(json, "vfov", 90.);

        let transform = Transform::read(json);

        let height = 2.0 * focal_distance * deg2rad(vfov / 2.0).tan();
        let width = resolution[0] / resolution[1] * height;
        let size = Vec2::new(width, -height);

        PinholeCamera {
            transform,
            size,
            focal_distance,
            resolution,
            aperture_radius,
        }
    }

    /// Generate a ray inside a given pixel
    pub fn generate_ray(&self, pixel: Vec2) -> Ray {
        let origin = Vec3::zeros();
        let xy = self
            .size
            .component_mul(&pixel)
            .component_div(&self.resolution)
            - self.size / 2.0;
        let direction = Vec3::new(xy.x, xy.y, -self.focal_distance);
        self.transform.ray(&Ray::new(origin, direction))
    }
}
