use nalgebra::{Vector2, Vector3};
use serde_json::{from_value, Value};
extern crate nalgebra_glm as glm;

use rand::Rng;

pub fn read_vector2_f32(v: &Value, name: &str, default: Vector2<f32>) -> Vector2<f32> {
    v.get(name).map_or(default, |v: &Value| {
        from_value::<Vector2<f32>>(v.clone()).unwrap()
    })
}

pub fn read_vector3_f32(v: &Value, name: &str, default: Vector3<f32>) -> Vector3<f32> {
    v.get(name).map_or(default, |v: &Value| {
        from_value::<Vector3<f32>>(v.clone()).unwrap()
    })
}

pub fn read_v_or_f(j: &Value, thing_name: &str, default: Vector3<f32>) -> Vector3<f32> {
    let v = j.get(thing_name).unwrap().clone();
    let thing: Vector3<f32>;
    if v.is_number() {
        let thing_number: f32 = from_value(v).unwrap();
        thing = Vector3::new(thing_number, thing_number, thing_number);
    } else {
        thing = read_vector3_f32(j, thing_name, default);
    }
    thing
}

pub trait Factory<T> {
    fn make(&mut self, v: &Value) -> Option<Vec<T>>;
}

// pub fn rad2deg(rad: f32) -> f32 {
//     180.0 / std::f32::consts::PI * rad
// }

pub fn deg2rad(rad: f32) -> f32 {
    std::f32::consts::PI / 180.0 * rad
}

// pub fn luminance(c: &Vector3<f32>) -> f32 {
//     glm::dot(c, &Vector3::new(0.212671, 0.715160, 0.072169))
// }

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vector3<f32> {
    let ones = Vector3::new(1.0, 1.0, 1.0);
    loop {
        let p = 2.0 * Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - ones;
        if p.norm_squared() < 1.0_ {
            return p;
        }
    }
}

// pub fn random_in_hemishere(rng: &mut impl Rng, normal: &Vector3<f32>) -> Vector3<f32> {
//     let in_unit_sphere = random_in_unit_sphere(rng);
//     if glm::dot(&in_unit_sphere, normal) > 0.0 {
//         in_unit_sphere
//     } else {
//         -1.0 * in_unit_sphere
//     }
// }

// pub fn random_in_unit_disk(rng: &mut impl Rng) -> Vector3<f32> {
//     loop {
//         let p = Vector3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
//         if p.norm_squared() < 1.0 {
//             return p;
//         }
//     }
// }

pub fn reflect(direction: &Vector3<f32>, normal: &Vector3<f32>) -> Vector3<f32> {
    direction - 2.0 * direction.dot(normal) * normal
}

pub fn refract(
    direction_in: &Vector3<f32>,
    normal: &Vector3<f32>,
    etai_over_etat: f32,
) -> Vector3<f32> {
    let cos_theta = glm::dot(&(-1.0 * direction_in), normal).min(1.0);
    let r_out_perp = etai_over_etat * (direction_in + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
