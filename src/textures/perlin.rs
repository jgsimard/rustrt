use rand::prelude::*;
// extern crate lazy_static;
use crate::sampling::random_in_unit_sphere;

extern crate nalgebra_glm as glm;
use glm::Vec3;

fn generate_perm(rng: &mut impl Rng) -> Vec<u8> {
    let mut p = Vec::with_capacity(256);
    for i in 0..=255 {
        p.push(i);
    }
    for i in (1..=255).rev() {
        p.swap(i, rng.gen_range(0..i));
    }
    p
}

fn generate_vecs(rng: &mut impl Rng) -> Vec<Vec3> {
    let mut f = Vec::with_capacity(256);
    for _ in 0..256 {
        f.push(random_in_unit_sphere(rng));
    }
    f
}

lazy_static::lazy_static! {
    pub static ref VECS: Vec<Vec3> = generate_vecs(&mut thread_rng());
    pub static ref PERM_X: Vec<u8> = generate_perm(&mut thread_rng());
    pub static ref PERM_Y: Vec<u8> = generate_perm(&mut thread_rng());
    pub static ref PERM_Z: Vec<u8> = generate_perm(&mut thread_rng());
}

#[allow(clippy::needless_range_loop)]
fn perlin_interp(corners: &[[[Vec3; 2]; 2]; 2], uvw: Vec3) -> f32 {
    let mut accum = 0.;
    let uvw3 = uvw
        .component_mul(&uvw)
        .component_mul(&(Vec3::new(3., 3., 3.) - 2. * uvw));
    let uvw3_inv = Vec3::new(1., 1., 1.) - uvw3;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let ijk = Vec3::new(i as f32, j as f32, k as f32);
                let weight = glm::dot(&corners[i][j][k], &(uvw - ijk));
                let ijk_inv = Vec3::new(1., 1., 1.) - ijk;
                accum += (ijk.component_mul(&uvw3) + ijk_inv.component_mul(&uvw3_inv)).product()
                    * weight;
            }
        }
    }
    accum
}

pub fn noise(p: Vec3, scale: f32) -> f32 {
    let p = p * scale;
    let ijk = p.map(f32::floor);
    let uvw = p - ijk;
    let mut corners = [[[Vec3::default(); 2]; 2]; 2];
    for di in 0..2 {
        for dj in 0..2 {
            for dk in 0..2 {
                let ix = PERM_X[((ijk.x as i32 + di as i32) & 255) as usize];
                let iy = PERM_Y[((ijk.y as i32 + dj as i32) & 255) as usize];
                let iz = PERM_Z[((ijk.z as i32 + dk as i32) & 255) as usize];
                corners[di][dj][dk] = VECS[(ix ^ iy ^ iz) as usize];
            }
        }
    }
    perlin_interp(&corners, uvw)
}

pub fn turbulant_noise(p: Vec3, scale_: f32, depth: usize) -> f32 {
    let mut accum = 0.;
    let mut weight = 1.;
    let mut scale = scale_;

    for _ in 0..depth {
        accum += weight * noise(p, scale);
        weight *= 0.5;
        scale *= 2.0;
    }
    f32::abs(accum)
}
