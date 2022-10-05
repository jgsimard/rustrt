use crate::ray::Ray;
use crate::utils::{deg2rad};

use nalgebra::{Vector3, Vector4, Matrix4, Matrix3x4};
use std::ops::{Mul};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value, json, from_value};



pub struct Transform{
    pub m: Matrix4<f32>,
    pub m_inv: Matrix4<f32>
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
        // (self.m * Vector4::new(v.x, v.y, v.z, 0.0)).xyz()
        (self.m * v.insert_row(3, 0.0)).xyz()
    }

    /// Apply the homogeneous transformation to a 3D normal
    pub fn normal(&self, n: &Vector3<f32>) -> Vector3<f32>{
        // (self.m_inv.transpose() * Vector4::new(n.x, n.y, n.z, 0.0)).xyz().normalize()
        (self.m_inv.transpose() * n.insert_row(3, 0.0)).xyz().normalize()
    }

    /// Transform a point by an arbitrary matrix in homogeneous coordinates
    pub fn point(&self, p: &Vector3<f32>) -> Vector3<f32>{
        // (self.m * Vector4::new(p.x, p.y, p.z, 1.0)).xyz()
        (self.m * p.insert_row(3, 1.0)).xyz()
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


 pub fn parse_transform(json: &Value) -> Transform{
    if json["transform"] != Value::Null {
        let t_json = json["transform"].clone();
        
        // multiple transforms
        if t_json.is_array() {
            let mut t: Transform = Default::default();
            for sub_t in t_json.as_array().unwrap(){
                t = parse_transform(&sub_t) * t;
            }
            return t;
        }
        println!("{}", "HERE".to_string());
        // single transform
        let kv: HashMap<String, serde_json::Value> = serde_json::from_value(t_json).unwrap();
        // wtf is this ?  unreadable
        if kv.contains_key("from") || kv.contains_key("at") || kv.contains_key("to") || kv.contains_key("up")
        {   
            println!("{}", "HERE 1".to_string());
            let from: Vector3<f32> = if kv.contains_key("from"){from_value(kv.get("from").unwrap().clone()).unwrap()} else {Vector3::z()};
            let to: Vector3<f32> = if kv.contains_key("to"){from_value(kv.get("to").unwrap().clone()).unwrap()} else {Vector3::zeros()};
            let at: Vector3<f32> = if kv.contains_key("at"){from_value(kv.get("at").unwrap().clone()).unwrap()} else {Vector3::zeros()};
            let up: Vector3<f32> = if kv.contains_key("up"){from_value(kv.get("up").unwrap().clone()).unwrap()} else {Vector3::y()};

            let dir = (from - at).normalize();
            let left = up.cross(&dir).normalize();
            let new_up = dir.cross(&left).normalize();
            
            
            let m = Matrix3x4::from_columns(&[left, new_up, dir, from]);
            let mut m = m.insert_row(3, 0.);
            m[(3,3)] = 1.0;

            return Transform::new(m);
        } 
        else if kv.contains_key("o") || kv.contains_key("x") || kv.contains_key("y") || kv.contains_key("z")
        {
            let o = if kv.contains_key("o"){from_value(kv.get("o").unwrap().clone()).unwrap()} else {Vector3::zeros()};
            let x = if kv.contains_key("x"){from_value(kv.get("x").unwrap().clone()).unwrap()} else {Vector3::x()};
            let y = if kv.contains_key("y"){from_value(kv.get("y").unwrap().clone()).unwrap()} else {Vector3::y()};
            let z = if kv.contains_key("z"){from_value(kv.get("z").unwrap().clone()).unwrap()} else {Vector3::z()};

            let m = Matrix3x4::from_columns(&[x, y, z, o]);
            let mut m = m.insert_row(3, 0.);
            m[(3,3)] = 1.0;

            return Transform::new(m);
        }
        else if kv.contains_key("translate") 
        {
            let v: Vector3<f32> = from_value(kv.get("translate").unwrap().clone()).unwrap();
            return Transform::new(Matrix4::new_translation(&v))
        }
        else if kv.contains_key("scale") 
        {   
            let scale = kv.get("scale").unwrap().clone();
            if scale.is_number(){
                let sn: f32 = from_value(scale).unwrap();
                return Transform::new(Matrix4::new_scaling(sn));
            }
            let sv: Vector3<f32> = from_value(scale).unwrap();
            Transform::new(Matrix4::new_nonuniform_scaling(&sv));
        }
        else if kv.contains_key("axis") || kv.contains_key("angle")
        {
            let axis: Vector3<f32> = if kv.contains_key("axis"){from_value(kv.get("axis").unwrap().clone()).unwrap()} else {Vector3::x()};
            let angle = if kv.contains_key("angle"){from_value(kv.get("angle").unwrap().clone()).unwrap()} else {0.0};
            let angle = deg2rad(angle);

            return Transform::new(Matrix4::from_scaled_axis(axis * angle));
            
        }
        else if kv.contains_key("matrix")
        {
            unimplemented!();
        }
        else{
            panic!("Unrecognized 'transform' command:")
        }
    } 
    Default::default()
 }
