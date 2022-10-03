use nalgebra::Vector3;

use rand::Rng;

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vector3<f32>{
    let unit = Vector3::new(1.0, 1.0, 0.0);
    loop{
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - unit;
        if p.norm() < 1.0_{
            return p
        }
    }
}

pub fn random_in_hemishere(rng: &mut impl Rng, normal: &Vector3<f32>) -> Vector3<f32>{
    let in_unit_sphere = random_in_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -1.0 * in_unit_sphere
    }
}

pub fn random_in_unit_disk(rng: &mut impl Rng) -> Vector3<f32>{
    loop {
        let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.norm_squared() < 1.0{
            return p;
        }
    }
}


pub fn reflect(direction: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32>{
    direction - 2.0 * direction.dot(normal) * normal
}

pub fn refract(direction_in: &Vector3<f32>, normal: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = (-1.0 * direction_in).dot(normal).min(1.0);
    let r_out_perp = etai_over_etat * (direction_in + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}