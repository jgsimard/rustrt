use nalgebra_glm::{cross, length, length2, Vec2, Vec3};
use rand::Rng;

use crate::core::utils::{lerp, sincos, FRAC_1_TWOPI, INV_FOURPI};

/// Uniformly sample a vector on a 2D disk with radius 1, centered around the origin
pub fn sample_disk(rv: Vec2) -> Vec2 {
    let r = f32::sqrt(rv.y);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    Vec2::new(cos_phi * r, sin_phi * r)
}

/// Probability density of `sample_disk`()
#[allow(unused)]
pub fn sample_disk_pdf(p: Vec2) -> f32 {
    if length2(&p) <= 1.0 {
        std::f32::consts::FRAC_1_PI
    } else {
        0.0
    }
}

/// Uniformly sample a vector on the unit sphere with respect to solid angles
pub fn sample_sphere(rv: Vec2) -> Vec3 {
    let cos_theta = 2.0 * rv.y - 1.0;
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}

/// Probability density of `sample_sphere`()
#[allow(unused)]
pub fn sample_sphere_pdf() -> f32 {
    INV_FOURPI
}

/// Uniformly sample a vector on the unit hemisphere around the pole (0,0,1) with respect to solid angles
pub fn sample_hemisphere(rv: Vec2) -> Vec3 {
    let cos_theta = rv.y;
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}

/// Probability density of `sample_hemisphere`()
pub fn sample_hemisphere_pdf(_v: &Vec3) -> f32 {
    FRAC_1_TWOPI
}

/// Uniformly sample a vector on the unit hemisphere around the pole (0,0,1) with respect to projected solid
/// angles
pub fn sample_hemisphere_cosine(rv: Vec2) -> Vec3 {
    let cos_theta = f32::sqrt(rv.y);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}

/// Probability density of `sample_hemisphere_cosine`()
#[allow(unused)]
pub fn sample_hemisphere_cosine_pdf(v: &Vec3) -> f32 {
    let cos_theta = v.z;
    cos_theta * std::f32::consts::FRAC_1_PI
}

/// Sample a vector on the unit hemisphere with a cosine-power density about the pole (0,0,1)
pub fn sample_hemisphere_cosine_power(exponent: f32, rv: Vec2) -> Vec3 {
    let cos_theta = f32::powf(rv.y, 1.0 / (exponent + 1.0));
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}

/// Probability density of `sample_hemisphere_cosine_power`()
pub fn sample_hemisphere_cosine_power_pdf(exponent: f32, cosine: f32) -> f32 {
    f32::powf(cosine, exponent) * (exponent + 1.0) * FRAC_1_TWOPI
}

/// Uniformly sample a vector on a spherical cap around (0, 0, 1)
///
/// A spherical cap is the subset of a unit sphere whose directions make an angle of less than 'theta' with the north
/// pole. This function expects the cosine of 'theta' as a parameter.
pub fn sample_sphere_cap(rv: Vec2, cos_theta_max: f32) -> Vec3 {
    let cos_theta = lerp(cos_theta_max, 1.0, rv.y);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}

/// Probability density of `sample_sphere_cap`()
pub fn sample_sphere_cap_pdf(_cos_theta: f32, cos_theta_max: f32) -> f32 {
    FRAC_1_TWOPI / (1.0 - cos_theta_max)
}

/// Sample a point uniformly on a triangle with vertices `v0`, `v1`, `v2`.
///
/// \param v0,v1,v2 The vertices of the triangle to sample
/// \param rv       Two random variables uniformly distributed in [0,1)
pub fn sample_triangle(v0: &Vec3, v1: &Vec3, v2: &Vec3, rv: Vec2) -> Vec3 {
    let mut a = rv.x;
    let mut b = rv.y;
    if (a + b) > 1.0 {
        a = 1.0 - a;
        b = 1.0 - b;
    }
    a * v0 + b * v1 + (1.0 - a - b) * v2
}

/// Sampling density of `sample_triangle`()
pub fn sample_triangle_pdf(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> f32 {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let area = 0.5 * length(&cross(&edge1, &edge2));
    1.0 / area
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
