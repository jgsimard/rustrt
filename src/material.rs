use nalgebra::Vector3;
use rand::Rng;


use super::ray::Ray;
use super::hit::HitRecord;
use super::utils::{random_in_unit_sphere, reflect, refract, reflectance};


pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)>;
}

pub struct  Lambertian{
    albedo: Vector3<f32>
}
impl Lambertian {
    pub fn new (a: Vector3<f32>) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let mut rng = rand::thread_rng();
        let mut scatter_direction = rec.p + random_in_unit_sphere(&mut rng).normalize();
        
        // Catch degenerate scatter direction
        const EPSILON: f32= 1.0e-8;
        if scatter_direction.norm() < EPSILON{
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Vector3<f32>,
    fuzz: f32
}

impl Metal {
    pub fn new(a: Vector3<f32>, fuzz: f32) -> Metal {
        Metal { albedo: a, fuzz: fuzz}
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let reflected = reflect(&r_in.direction, &rec.normal).normalize();
        
        let mut rng = rand::thread_rng();
        
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(&mut rng));

        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f32
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            ir: index_of_refraction
        }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();

        let cos_theta = ((-1.0) * unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        
        let mut rng = rand::thread_rng();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f32>() < reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        
        let scattered = Ray::new(rec.p, direction);

        Some((Vector3::new(1.0, 1.0, 1.0), scattered))
    }
}