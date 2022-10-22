extern crate nalgebra_glm as glm;
use glm::{Vec2, Vec3};
use rand::Rng;
use serde_json::{from_value, Value};
use std::ops::{Add, Mul, Sub};

pub const INV_FOURPI: f32 = 1.0 / (4.0 * std::f32::consts::PI);
pub const FRAC_1_TWOPI: f32 = 1.0 / (2.0 * std::f32::consts::PI);

pub fn sincos(x: f32) -> (f32, f32) {
    (f32::sin(x), f32::cos(x))
}

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

#[allow(unused)]
pub fn lerp<T, F>(a: T, b: T, f: F) -> T
where
    T: Clone + Add<T, Output = T> + Sub<T, Output = T> + Mul<F, Output = T>,
{
    a.clone() + (b - a) * f
}

pub fn direction_to_spherical_coordinates(v: &Vec3) -> Vec2 {
    Vec2::new(
        f32::atan2(-v.y, -v.x) + std::f32::consts::PI,
        f32::acos(v.z),
    )
}

pub fn direction_to_spherical_uv(p: &Vec3) -> Vec2 {
    let sph = direction_to_spherical_coordinates(p);
    Vec2::new(
        modulo(sph.x * FRAC_1_TWOPI - 0.5, 1.0),
        1.0 - sph.y * std::f32::consts::FRAC_1_PI,
    )
}

pub fn spherical_coordinates_to_direction(phi_theta: &Vec2) -> Vec3 {
    let (sin_theta, cos_theta) = sincos(phi_theta.y);
    let (sin_phi, cos_phi) = sincos(phi_theta.x);

    return Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta);
}

#[allow(unused)]
pub fn spherical_uv_to_direction(uv: &Vec2) -> Vec3 {
    return spherical_coordinates_to_direction(&Vec2::new(
        (uv.x - 0.5) * 2.0 * std::f32::consts::PI,
        (1.0 - uv.y) * std::f32::consts::PI,
    ));
}

pub fn read<T: for<'de> serde::de::Deserialize<'de>>(v: &Value, name: &str) -> T {
    from_value::<T>(
        (*v.get(name)
            .unwrap_or_else(|| panic!("could not read {}", name)))
        .clone(),
    )
    .unwrap_or_else(|_v| panic!("could not transform {}", name))
}

pub fn read_or<T: for<'de> serde::de::Deserialize<'de>>(v: &Value, name: &str, default: T) -> T {
    v.get(name)
        .map_or(default, |v: &Value| from_value::<T>(v.clone()).unwrap())
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

#[allow(unused)]
pub fn rad2deg(rad: f32) -> f32 {
    180.0 / std::f32::consts::PI * rad
}

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

// COLORS
#[allow(unused)]
pub fn viridis(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(0.2777273272234177, 0.005407344544966578, 0.3340998053353061);
    const C1: Vec3 = Vec3::new(0.1050930431085774, 1.404613529898575, 1.384590162594685);
    const C2: Vec3 = Vec3::new(-0.3308618287255563, 0.214847559468213, 0.09509516302823659);
    const C3: Vec3 = Vec3::new(-4.634230498983486, -5.799100973351585, -19.33244095627987);
    const C4: Vec3 = Vec3::new(6.228269936347081, 14.17993336680509, 56.69055260068105);
    const C5: Vec3 = Vec3::new(4.776384997670288, -13.74514537774601, -65.35303263337234);
    const C6: Vec3 = Vec3::new(-5.435455855934631, 4.645852612178535, 26.3124352495832);

    return C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))));
}

pub fn inferno(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(0.00021894036911922, 0.0016510046310010, -0.019480898437091);
    const C1: Vec3 = Vec3::new(0.1065134194856116, 0.5639564367884091, 3.932712388889277);
    const C2: Vec3 = Vec3::new(11.60249308247187, -3.972853965665698, -15.9423941062914);
    const C3: Vec3 = Vec3::new(-41.70399613139459, 17.43639888205313, 44.35414519872813);
    const C4: Vec3 = Vec3::new(77.162935699427, -33.40235894210092, -81.80730925738993);
    const C5: Vec3 = Vec3::new(-71.31942824499214, 32.62606426397723, 73.20951985803202);
    const C6: Vec3 = Vec3::new(25.13112622477341, -12.24266895238567, -23.07032500287172);

    return C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))));
}

#[allow(unused)]
pub fn magma(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(-0.002136485053939, -0.000749655052795, -0.005386127855323);
    const C1: Vec3 = Vec3::new(0.2516605407371642, 0.6775232436837668, 2.494026599312351);
    const C2: Vec3 = Vec3::new(8.353717279216625, -3.577719514958484, 0.3144679030132573);
    const C3: Vec3 = Vec3::new(-27.66873308576866, 14.26473078096533, -13.64921318813922);
    const C4: Vec3 = Vec3::new(52.17613981234068, -27.94360607168351, 12.94416944238394);
    const C5: Vec3 = Vec3::new(-50.76852536473588, 29.04658282127291, 4.23415299384598);
    const C6: Vec3 = Vec3::new(18.65570506591883, -11.48977351997711, -5.601961508734096);

    return C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))));
}

#[allow(unused)]
pub fn plasma(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(0.05873234392399702, 0.02333670892565664, 0.5433401826748754);
    const C1: Vec3 = Vec3::new(2.176514634195958, 0.2383834171260182, 0.7539604599784036);
    const C2: Vec3 = Vec3::new(-2.689460476458034, -7.455851135738909, 3.110799939717086);
    const C3: Vec3 = Vec3::new(6.130348345893603, 42.3461881477227, -28.51885465332158);
    const C4: Vec3 = Vec3::new(-11.10743619062271, -82.66631109428045, 60.13984767418263);
    const C5: Vec3 = Vec3::new(10.02306557647065, 71.41361770095349, -54.07218655560067);
    const C6: Vec3 = Vec3::new(-3.658713842777788, -22.93153465461149, 18.19190778539828);

    return C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))));
}
