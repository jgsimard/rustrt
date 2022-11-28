use crate::core::aabb::Aabb;
use crate::core::ray::Ray;
use crate::core::utils::{deg2rad, read_or};

use nalgebra_glm::{cross, normalize, Mat3x4, Mat4, Vec3};
use serde_json::{from_value, Value};
use std::ops::Mul;

/// Homogeneous coordinate transformation
///
/// This class stores a general homogeneous coordinate transformation, such as rotation, translation, uniform or
/// non-uniform scaling, and perspective transformations. The inverse of this transformation is also recorded here,
/// since it is required when transforming normal vectors.

#[derive(Debug, PartialEq, Clone)]
pub struct Transform {
    pub m: Mat4,
    pub m_inv: Mat4,
}

impl Transform {
    pub fn new(m: Mat4) -> Transform {
        Transform {
            m,
            m_inv: m.try_inverse().expect("Matrix not invertible"),
        }
    }

    /// Return the inverse transformation
    pub fn inverse(&self) -> Transform {
        Transform {
            m: self.m_inv,
            m_inv: self.m,
        }
    }

    /// Apply the homogeneous transformation to a 3D direction vector
    pub fn vector(&self, v: &Vec3) -> Vec3 {
        (self.m * v.insert_row(3, 0.0)).xyz()
    }

    /// Apply the homogeneous transformation to a 3D normal
    pub fn normal(&self, n: &Vec3) -> Vec3 {
        (self.m_inv.transpose() * n.insert_row(3, 0.0))
            .xyz()
            .normalize()
    }

    /// Transform a point by an arbitrary matrix in homogeneous coordinates
    pub fn point(&self, p: &Vec3) -> Vec3 {
        (self.m * p.insert_row(3, 1.0)).xyz()
    }

    /// Apply the homogeneous transformation to a Ray
    pub fn ray(&self, r: &Ray) -> Ray {
        Ray {
            origin: self.point(&r.origin),
            direction: self.vector(&r.direction),
            min_t: r.min_t,
            max_t: r.max_t,
        }
    }

    /// Transform the axis-aligned Box and return the bounding box of the result
    pub fn aabb(&self, box3: &Aabb) -> Aabb {
        // a transformed empty box is still empty
        if box3.is_empty() {
            return (*box3).clone();
        }

        // create the transformed bounding box
        let mut bb = Aabb::new();
        bb.enclose_point(&self.point(&box3.min));
        bb.enclose_point(&self.point(&box3.max));

        bb
    }

    pub fn axis_offset(x: &Vec3, y: &Vec3, z: &Vec3, o: &Vec3) -> Transform {
        let m = Mat3x4::from_columns(&[*x, *y, *z, *o]);
        let mut m = m.insert_row(3, 0.);
        m[(3, 3)] = 1.0;
        Transform::new(m)
    }

    pub fn translate(t: &Vec3) -> Transform {
        Transform::new(Mat4::new_translation(t))
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
            m: Mat4::identity(),
            m_inv: Mat4::identity(),
        }
    }
}

pub fn parse_transform(json: &Value) -> Transform {
    // multiple transforms
    if json.is_array() {
        let mut t: Transform = Transform::default();
        for sub_t in json.as_array().unwrap() {
            t = parse_transform(sub_t) * t;
        }
        return t;
    }
    // single transform
    let json_map = json.as_object().unwrap();

    if json_map.contains_key("from")
        || json_map.contains_key("at")
        || json_map.contains_key("to")
        || json_map.contains_key("up")
    {
        let from = read_or(json, "from", Vec3::z());
        let to = read_or(json, "to", Vec3::zeros());
        let at = read_or(json, "at", Vec3::zeros()) + to;
        let up = read_or(json, "up", Vec3::y());

        let dir = normalize(&(from - at));
        let left = normalize(&cross(&up, &dir));
        let new_up = normalize(&cross(&dir, &left));

        Transform::axis_offset(&left, &new_up, &dir, &from)
    } else if json_map.contains_key("o")
        || json_map.contains_key("x")
        || json_map.contains_key("y")
        || json_map.contains_key("z")
    {
        let o = read_or(json, "o", Vec3::zeros());
        let x = read_or(json, "x", Vec3::x());
        let y = read_or(json, "y", Vec3::y());
        let z = read_or(json, "z", Vec3::z());

        Transform::axis_offset(&x, &y, &z, &o)
    } else if json_map.contains_key("translate") {
        let t = read_or(json, "translate", Vec3::zeros());
        Transform::translate(&t)
    } else if json_map.contains_key("scale") {
        let scale = json_map.get("scale").unwrap().clone();
        if scale.is_number() {
            let sn: f32 = from_value(scale).expect("could not load 'scale' number Transform");
            return Transform::new(Mat4::new_scaling(sn));
        }
        let sv: Vec3 = from_value(scale).expect("could not load 'scale' vector Transform");
        Transform::new(Mat4::new_nonuniform_scaling(&sv))
    } else if json_map.contains_key("axis") || json_map.contains_key("angle") {
        let axis = read_or(json, "axis", Vec3::x());
        let angle = deg2rad(read_or(json, "angle", 0.0));
        Transform::new(Mat4::from_scaled_axis(axis * angle))
    } else if json_map.contains_key("matrix") {
        unimplemented!();
    } else {
        panic!("Unrecognized 'transform' command:")
    }
}

pub fn read_transform(v: &Value) -> Transform {
    if let Some(transform) = v.get("transform") {
        parse_transform(transform)
    } else {
        Transform::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::ray::Ray;
    use crate::core::transform::{read_transform, Transform};
    use nalgebra::{Matrix4, Vector3};
    use nalgebra_glm::Mat4;
    use serde_json::json;

    #[test]
    fn parse_from_at_to_up() {
        let transform_json = json!({
            "transform":{
                "from": [-10.0, 10.0, 40.0],
                "to": [0.0, -1.0, 0.0],
                "up": [0.0, 1.0, 0.0]
            }
        });
        let transform = read_transform(&transform_json);
        let m = Matrix4::new(
            0.970142, 0.062519, -0.234339, -10.0, 0.0, 0.966205, 0.257773, 10.0, 0.242535,
            -0.250076, 0.937357, 40.0, 0.0, 0.0, 0.0, 1.0,
        );
        approx::assert_abs_diff_eq!(m, transform.m, epsilon = 1e-5);
    }

    #[test]
    fn inverse() {
        let transformation_matrix = Matrix4::new(
            -0.846852, -0.492958, -0.199586, -0.997497, 0.107965, -0.526819, 0.843093, 0.127171,
            -0.520755, 0.692427, 0.499359, -0.613392, 0.0, 0.0, 0.0, 1.0,
        );
        let identity = Mat4::identity();
        let transform = Transform::new(transformation_matrix);
        let transform_inverse = transform.inverse();

        let res = transform.clone() * transform_inverse.clone();
        approx::assert_abs_diff_eq!(identity, res.m, epsilon = 1e-5);
        approx::assert_abs_diff_eq!(identity, res.m_inv, epsilon = 1e-5);

        let res = transform_inverse * transform;
        approx::assert_abs_diff_eq!(identity, res.m, epsilon = 1e-5);
        approx::assert_abs_diff_eq!(identity, res.m_inv, epsilon = 1e-5);
    }

    #[test]
    fn from_transformation_matrix() {
        // Setup
        let transformation_matrix = Matrix4::new(
            -0.846852, -0.492958, -0.199586, -0.997497, 0.107965, -0.526819, 0.843093, 0.127171,
            -0.520755, 0.692427, 0.499359, -0.613392, 0.0, 0.0, 0.0, 1.0,
        );

        let transform = Transform::new(transformation_matrix);

        let vector = Vector3::new(-0.997497, 0.127171, -0.613_392);
        let point = Vector3::new(0.617481, 0.170019, -0.0402539);
        let normal = Vector3::new(-0.281208, 0.743764, 0.606_413);
        let ray = Ray::new(
            Vector3::new(-0.997497, 0.127171, -0.613392),
            Vector3::new(0.962222, 0.264941, -0.0627278),
        );

        // Use Transform
        let transformed_vector = transform.vector(&vector);
        let transformed_point = transform.point(&point);
        let transformed_normal = transform.normal(&normal);
        let transformed_ray = transform.ray(&ray);

        // Test Transform
        let correct_transformed_vector = Vector3::new(0.904467, -0.691_837, 0.301205);
        let correct_transformed_point = Vector3::new(-1.596_19, 0.0703303, -0.837324);
        let correct_transformed_normal = Vector3::new(-0.249534, 0.0890737, 0.96426);
        let correct_transformed_ray_position = Vector3::new(-0.0930302, -0.564666, -0.312187);
        let correct_transformed_ray_direction = Vector3::new(-0.932945, -0.088575, -0.348953);

        approx::assert_abs_diff_eq!(
            correct_transformed_vector,
            transformed_vector,
            epsilon = 1e-5
        );
        approx::assert_abs_diff_eq!(correct_transformed_point, transformed_point, epsilon = 1e-5);
        approx::assert_abs_diff_eq!(
            correct_transformed_normal,
            transformed_normal,
            epsilon = 1e-5
        );

        approx::assert_abs_diff_eq!(
            correct_transformed_ray_position,
            transformed_ray.origin,
            epsilon = 1e-5
        );
        approx::assert_abs_diff_eq!(
            correct_transformed_ray_direction,
            transformed_ray.direction,
            epsilon = 1e-5
        );
    }
}
