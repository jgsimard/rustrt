use nalgebra::Vector3;


use super::ray::Ray;
use super::utils::random_in_unit_disk;


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
