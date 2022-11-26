use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use nalgebra_glm::{dot, Vec2, Vec3};
use serde_json::{from_value, Value};
use std::fmt::Write;
use std::ops::{Add, Mul, Sub};
use std::sync::atomic::AtomicUsize;

pub const INV_FOURPI: f32 = 1.0 / (4.0 * std::f32::consts::PI);
pub const FRAC_1_TWOPI: f32 = 1.0 / (2.0 * std::f32::consts::PI);
pub static RAYS: AtomicUsize = AtomicUsize::new(0);
pub static INTERSECTION_TEST: AtomicUsize = AtomicUsize::new(0);

pub fn get_progress_bar(size: usize) -> ProgressBar {
    let progress_bar = ProgressBar::new(size as u64);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap();
        })
        .progress_chars("#>-"),
    );
    progress_bar
}
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

    a
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

pub fn spherical_coordinates_to_direction(phi_theta: Vec2) -> Vec3 {
    let (sin_theta, cos_theta) = sincos(phi_theta.y);
    let (sin_phi, cos_phi) = sincos(phi_theta.x);

    Vec3::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta)
}

#[allow(unused)]
pub fn spherical_uv_to_direction(uv: Vec2) -> Vec3 {
    spherical_coordinates_to_direction(Vec2::new(
        (uv.x - 0.5) * 2.0 * std::f32::consts::PI,
        (1.0 - uv.y) * std::f32::consts::PI,
    ))
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
    if v.is_number() {
        let thing_number: f32 = from_value(v).unwrap();
        Vec3::new(thing_number, thing_number, thing_number)
    } else {
        read::<Vec3>(j, thing_name)
    }
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
    dot(c, &Vec3::new(0.212671, 0.715160, 0.072169))
}

pub fn reflect(direction: &Vec3, normal: &Vec3) -> Vec3 {
    direction - 2.0 * direction.dot(normal) * normal
}

pub fn refract(direction_in: &Vec3, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(&(-1.0 * direction_in), normal).min(1.0);
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
    const C0: Vec3 = Vec3::new(0.277_727, 0.005_407, 0.334_099);
    const C1: Vec3 = Vec3::new(0.105_093, 1.404_613, 1.384_59);
    const C2: Vec3 = Vec3::new(-0.330_861, 0.214_847, 0.095_095);
    const C3: Vec3 = Vec3::new(-4.634_23, -5.799_1, -19.332_44);
    const C4: Vec3 = Vec3::new(6.228_269, 14.179_933, 56.690_55);
    const C5: Vec3 = Vec3::new(4.776_384, -13.745_145, -65.353_032);
    const C6: Vec3 = Vec3::new(-5.435_455, 4.645_852, 26.312_435);

    C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))))
}

#[allow(unused)]
pub fn inferno(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(0.000_218, 0.001_651, -0.019_480);
    const C1: Vec3 = Vec3::new(0.106_513, 0.563_956, 3.932_712);
    const C2: Vec3 = Vec3::new(11.602_49, -3.972_853, -15.942_39);
    const C3: Vec3 = Vec3::new(-41.703_99, 17.436_39, 44.354_14);
    const C4: Vec3 = Vec3::new(77.162_93, -33.402_35, -81.807_3);
    const C5: Vec3 = Vec3::new(-71.319_42, 32.626_06, 73.209_51);
    const C6: Vec3 = Vec3::new(25.131_12, -12.242_66, -23.070_32);

    C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))))
}

#[allow(unused)]
pub fn magma(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(-0.002_136, -0.000_749, -0.005_386);
    const C1: Vec3 = Vec3::new(0.251_660, 0.677_523, 2.494_026);
    const C2: Vec3 = Vec3::new(8.353_717, -3.577_719, 0.314_467);
    const C3: Vec3 = Vec3::new(-27.668_733, 14.264_73, -13.649_213);
    const C4: Vec3 = Vec3::new(52.176_13, -27.943_606, 12.944_169);
    const C5: Vec3 = Vec3::new(-50.768_525, 29.046_582, 4.234_152);
    const C6: Vec3 = Vec3::new(18.655_705, -11.489_773, -5.601_961);

    C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))))
}

#[allow(unused)]
pub fn plasma(t: f32) -> Vec3 {
    const C0: Vec3 = Vec3::new(0.058_732, 0.023_336, 0.543_340);
    const C1: Vec3 = Vec3::new(2.176_514, 0.238_383, 0.753_960);
    const C2: Vec3 = Vec3::new(-2.689_46, -7.455_851, 3.110_799);
    const C3: Vec3 = Vec3::new(6.130_348, 42.346_18, -28.518_85);
    const C4: Vec3 = Vec3::new(-11.107_43, -82.666_31, 60.139_84);
    const C5: Vec3 = Vec3::new(10.023_06, 71.413_61, -54.072_18);
    const C6: Vec3 = Vec3::new(-3.658_714, -22.931_53, 18.191_9);

    C0 + t * (C1 + t * (C2 + t * (C3 + t * (C4 + t * (C5 + t * C6)))))
}
