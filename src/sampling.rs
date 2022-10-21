extern crate nalgebra_glm as glm;
use crate::utils::{lerp, sincos, INV_FOURPI, INV_TWOPI};
use glm::{Vec2, Vec3};
// let mut rng = rand::thread_rng();

/// Uniformly sample a vector on a 2D disk with radius 1, centered around the origin
fn sample_disk(rv: &Vec2) -> Vec2 {
    let r = f32::sqrt(rv.y);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    return Vec2::new(cos_phi * r, sin_phi * r);
}

/// Probability density of #sample_disk()
fn sample_disk_pdf(p: &Vec2) -> f32 {
    if glm::length2(p) <= 1.0 {
        std::f32::consts::FRAC_1_PI
    } else {
        0.0
    }
}

/// Uniformly sample a vector on the unit sphere with respect to solid angles
fn sample_sphere(rv: &Vec2) -> Vec3 {
    let cos_theta = 2.0 * rv.y - 1.0;
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    return Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta);
}

/// Probability density of #sample_sphere()
pub fn sample_sphere_pdf() -> f32 {
    return INV_FOURPI;
}

/// Uniformly sample a vector on the unit hemisphere around the pole (0,0,1) with respect to solid angles
pub fn sample_hemisphere(rv: &Vec2) -> Vec3 {
    let cos_theta = rv.y;
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    return Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta);
}

/// Probability density of #sample_hemisphere()
pub fn sample_hemisphere_pdf(v: &Vec3) -> f32 {
    return INV_TWOPI;
}

/// Uniformly sample a vector on the unit hemisphere around the pole (0,0,1) with respect to projected solid
/// angles
pub fn sample_hemisphere_cosine(rv: &Vec2) -> Vec3 {
    let cos_theta = f32::sqrt(rv.y);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    return Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta);
}

/// Probability density of #sample_hemisphere_cosine()
pub fn sample_hemisphere_cosine_pdf(v: &Vec3) -> f32 {
    let cos_theta = v.z;
    return cos_theta * std::f32::consts::FRAC_1_PI;
}

/// Sample a vector on the unit hemisphere with a cosine-power density about the pole (0,0,1)
pub fn sample_hemisphere_cosine_power(exponent: f32, rv: &Vec2) -> Vec3 {
    let cos_theta = f32::powf(rv.y, 1.0 / (exponent + 1.0));
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    return Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta);
}

/// Probability density of #sample_hemisphere_cosine_power()
pub fn sample_hemisphere_cosine_power_pdf(exponent: f32, cosine: f32) -> f32 {
    // from cst * \int_0^{2pi} \int_0^{pi/2} cos^n(theta) sin(theta) dtheta dphi =  cst * 2 pi / (n + 1) = 1 => cst = (n + 1) / (2 pi)
    return f32::powf(cosine, exponent) * (exponent + 1.0) * INV_TWOPI;
}

/// Uniformly sample a vector on a spherical cap around (0, 0, 1)
///
/// A spherical cap is the subset of a unit sphere whose directions make an angle of less than 'theta' with the north
/// pole. This function expects the cosine of 'theta' as a parameter.
pub fn sample_sphere_cap(rv: &Vec2, cos_theta_max: f32) -> Vec3 {
    let cos_theta = lerp(cos_theta_max, 1.0, rv.y);
    let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
    let (sin_phi, cos_phi) = sincos(std::f32::consts::TAU * rv.x);
    return Vec3::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta);
}

/// Probability density of #sample_sphere_cap()
pub fn sample_sphere_cap_pdf(cos_theta: f32, cos_theta_max: f32) -> f32 {
    return INV_TWOPI / (1.0 - cos_theta_max);
}

/// Sample a point uniformly on a triangle with vertices `v0`, `v1`, `v2`.
///
/// \param v0,v1,v2 The vertices of the triangle to sample
/// \param rv       Two random variables uniformly distributed in [0,1)
pub fn sample_triangle(v0: &Vec3, v1: &Vec3, v2: &Vec3, rv: &Vec2) -> Vec3 {
    let mut a = rv.x;
    let mut b = rv.y;
    if (a + b) > 1.0 {
        a = 1.0 - a;
        b = 1.0 - b;
    }
    return a * v0 + b * v1 + (1.0 - a - b) * v2;
}

/// Sampling density of #sample_triangle()
pub fn sample_triangle_pdf(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> f32 {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    let area = 0.5 * glm::length(&glm::cross(&edge1, &edge2));
    return 1.0 / area;
}
