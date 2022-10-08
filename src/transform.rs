use crate::ray::Ray;
use crate::utils::deg2rad;

extern crate nalgebra_glm as glm;

use nalgebra::{Matrix3x4, Matrix4, Vector3};
use serde_json::{from_value, Value};
use std::collections::HashMap;
use std::ops::Mul;

#[derive(Debug)]
pub struct Transform {
    pub m: Matrix4<f32>,
    pub m_inv: Matrix4<f32>,
}

impl Transform {
    pub fn new(m: Matrix4<f32>) -> Transform {
        Transform {
            m: m,
            m_inv: m.try_inverse().expect("Matrix not invertible"),
        }
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    /// Apply the homogeneous transformation to a 3D direction vector
    pub fn vector(&self, v: &Vector3<f32>) -> Vector3<f32> {
        // (self.m * Vector4::new(v.x, v.y, v.z, 0.0)).xyz()
        (self.m * v.insert_row(3, 0.0)).xyz()
    }

    /// Apply the homogeneous transformation to a 3D normal
    pub fn normal(&self, n: &Vector3<f32>) -> Vector3<f32> {
        // (self.m_inv.transpose() * Vector4::new(n.x, n.y, n.z, 0.0)).xyz().normalize()
        (self.m_inv.transpose() * n.insert_row(3, 0.0))
            .xyz()
            .normalize()
    }

    /// Transform a point by an arbitrary matrix in homogeneous coordinates
    pub fn point(&self, p: &Vector3<f32>) -> Vector3<f32> {
        // (self.m * Vector4::new(p.x, p.y, p.z, 1.0)).xyz()
        (self.m * p.insert_row(3, 1.0)).xyz()
    }

    /// Apply the homogeneous transformation to a Ray
    pub fn ray(&self, r: &Ray) -> Ray {
        Ray {
            origin: self.point(&r.origin),
            direction: self.vector(&r.direction),
            mint: r.mint,
            maxt: r.maxt,
        }
    }
    pub fn axis_offset(
        x: &Vector3<f32>,
        y: &Vector3<f32>,
        z: &Vector3<f32>,
        o: &Vector3<f32>,
    ) -> Transform {
        let m = Matrix3x4::from_columns(&[*x, *y, *z, *o]);
        let mut m = m.insert_row(3, 0.);
        m[(3, 3)] = 1.0;
        Transform::new(m)
    }

    pub fn translate(t: &Vector3<f32>) -> Transform {
        Transform::new(Matrix4::new_translation(&t))
    }
}

impl Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        Transform {
            m: self.m * other.m,
            m_inv: other.m_inv * self.m_inv,
        }
    }
}

impl Default for Transform {
    fn default() -> Transform {
        Transform {
            m: Matrix4::identity(),
            m_inv: Matrix4::identity(),
        }
    }
}

pub fn parse_transform(json: &Value) -> Transform {
    if json["transform"] != Value::Null {
        let t_json = json["transform"].clone();

        // multiple transforms
        if t_json.is_array() {
            let mut t: Transform = Default::default();
            for sub_t in t_json.as_array().unwrap() {
                t = parse_transform(&sub_t) * t;
            }
            return t;
        }
        // single transform
        let kv = t_json.as_object().unwrap();
        let kv: HashMap<String, serde_json::Value> = serde_json::from_value(t_json).unwrap();
        let read_vector3 = |v: &Value| from_value::<Vector3<f32>>(v.clone()).unwrap();
        let read = |s: &str, default| kv.get(s).map_or(default, read_vector3);

        if kv.contains_key("from")
            || kv.contains_key("at")
            || kv.contains_key("to")
            || kv.contains_key("up")
        {
            let from = read("from", Vector3::z());
            let to = read("to", Vector3::zeros());
            let at = read("at", Vector3::zeros()) + to;
            let up = read("up", Vector3::y());

            let dir = glm::normalize(&(from - at));
            let left = glm::normalize(&glm::cross(&up, &dir));
            let new_up = glm::normalize(&glm::cross(&dir, &left));

            return Transform::axis_offset(&left, &new_up, &dir, &from);
        } else if kv.contains_key("o")
            || kv.contains_key("x")
            || kv.contains_key("y")
            || kv.contains_key("z")
        {
            let o = read("o", Vector3::zeros());
            let x = read("x", Vector3::x());
            let y = read("y", Vector3::y());
            let z = read("z", Vector3::z());

            return Transform::axis_offset(&x, &y, &z, &o);
        } else if kv.contains_key("translate") {
            let t = read("translate", Vector3::zeros());
            return Transform::new(Matrix4::new_translation(&t));
        } else if kv.contains_key("scale") {
            let scale = kv.get("scale").unwrap().clone();
            if scale.is_number() {
                let sn: f32 = from_value(scale).expect("could not load 'scale' number Transform");
                return Transform::new(Matrix4::new_scaling(sn));
            }
            let sv: Vector3<f32> =
                from_value(scale).expect("could not load 'scale' vector Transform");
            return Transform::new(Matrix4::new_nonuniform_scaling(&sv));
        } else if kv.contains_key("axis") || kv.contains_key("angle") {
            let axis = read("axis", Vector3::x());
            let angle = kv
                .get("angle")
                .map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());
            let angle = deg2rad(angle);

            return Transform::new(Matrix4::from_scaled_axis(axis * angle));
        } else if kv.contains_key("matrix") {
            unimplemented!();
        } else {
            panic!("Unrecognized 'transform' command:")
        }
    }
    Default::default()
}
