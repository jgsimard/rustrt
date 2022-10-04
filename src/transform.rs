use super::ray::Ray;

use nalgebra::{Vector3, Vector4, Matrix4};
use std::ops::{Mul};


pub struct Transform{
    m: Matrix4<f32>,
    m_inv: Matrix4<f32>
}

impl Transform{
    
    pub fn new(m: Matrix4<f32>) -> Transform{
        Transform { 
            m: m, 
            m_inv: m.try_inverse().expect("Matrix not invertible")
        }
    }

    pub fn inverse(&self) -> Transform{
        Transform{
            m: self.m_inv,
            m_inv: self.m
        }
    }

    /// Apply the homogeneous transformation to a 3D direction vector
    pub fn vector(&self, v: &Vector3<f32>) -> Vector3<f32>{
        (self.m * Vector4::new(v.x, v.y, v.z, 0.0)).xyz()
    }

    /// Apply the homogeneous transformation to a 3D normal
    pub fn normal(&self, n: &Vector3<f32>) -> Vector3<f32>{
        (self.m_inv.transpose() * Vector4::new(n.x, n.y, n.z, 0.0)).xyz().normalize()
    }

    /// Transform a point by an arbitrary matrix in homogeneous coordinates
    pub fn point(&self, p: &Vector3<f32>) -> Vector3<f32>{
        (self.m * Vector4::new(p.x, p.y, p.z, 1.0)).xyz()
    }

    /// Apply the homogeneous transformation to a ray
    pub fn ray(&self, r: &Ray) -> Ray {
        Ray { 
            origin: self.point(&r.origin), 
            direction: self.vector(&r.direction),
            mint: r.mint, 
            maxt: r.mint  
        }
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform{
        Transform{
            m: self.m * other.m,
            m_inv : other.m_inv * self.m_inv
        }
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform{m:  Matrix4::identity(), m_inv: Matrix4::identity()}   
    }
}
