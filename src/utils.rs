extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use rand::Rng;
use serde_json::{from_value, Value};
use std::ops::{Add, Mul, Sub};

pub const INV_FOURPI: f32 = 1.0 / (4.0 * std::f32::consts::PI);
pub const INV_TWOPI: f32 = 1.0 / (2.0 * std::f32::consts::PI);

/// Always-positive modulo operation
fn modulo(a_: f32, b: f32) -> f32 {
    let mut a = a_;
    let n = (a / b).floor();
    a -= n * b;
    if a < 0.0 {
        a += b;
    }

    return a;
}

pub fn lerp<T, F>(a: T, b: T, f: F) -> T
where
    T: Clone + Add<T, Output = T> + Sub<T, Output = T> + Mul<F, Output = T>,
{
    a.clone() + (b - a) * f
}

fn direction_to_spherical_coordinates(v: &Vec3) -> Vec2 {
    Vec2::new(
        f32::atan2(-v.y, -v.x) + std::f32::consts::PI,
        f32::acos(v.z),
    )
}

pub fn direction_to_spherical_uv(p: &Vec3) -> Vec2 {
    let sph = direction_to_spherical_coordinates(p);
    Vec2::new(
        modulo(sph.x * INV_TWOPI - 0.5, 1.0),
        1.0 - sph.y * std::f32::consts::FRAC_1_PI,
    )
}

pub fn read<T: for<'de> serde::de::Deserialize<'de>>(v: &Value, name: &str) -> T {
    from_value::<T>(
        (*v.get(name)
            .unwrap_or_else(|| panic!("could not read {}", name)))
        .clone(),
    )
    .unwrap_or_else(|_v| panic!("could not transform {}", name))
}

pub fn read_v_or_f(j: &Value, thing_name: &str) -> Vec3 {
    let v = j.get(thing_name).unwrap().clone();
    let thing: Vec3;
    if v.is_number() {
        let thing_number: f32 = from_value(v).unwrap();
        thing = Vec3::new(thing_number, thing_number, thing_number);
    } else {
        thing = read::<Vec3>(j, thing_name);
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

pub fn luminance(c: &Vec3) -> f32 {
    glm::dot(c, &Vec3::new(0.212671, 0.715160, 0.072169))
}

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    const ONES: Vec3 = Vec3::new(1.0, 1.0, 1.0);
    loop {
        let rand_vec = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        let p = 2.0 * rand_vec - ONES;
        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

// pub fn random_in_hemishere(rng: &mut impl Rng, normal: &Vec3) -> Vec3 {
//     let in_unit_sphere = random_in_unit_sphere(rng);
//     if glm::dot(&in_unit_sphere, normal) > 0.0 {
//         in_unit_sphere
//     } else {
//         -1.0 * in_unit_sphere
//     }
// }

// pub fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
//     loop {
//         let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
//         if p.norm_squared() < 1.0 {
//             return p;
//         }
//     }
// }

pub fn reflect(direction: &Vec3, normal: &Vec3) -> Vec3 {
    direction - 2.0 * direction.dot(normal) * normal
}

pub fn refract(direction_in: &Vec3, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = glm::dot(&(-1.0 * direction_in), normal).min(1.0);
    let r_out_perp = etai_over_etat * (direction_in + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

/// Use Schlick's approximation for reflectance
pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
