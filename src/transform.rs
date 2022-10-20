use crate::aabb::Aabb;
use crate::ray::Ray;
use crate::utils::deg2rad;

extern crate nalgebra_glm as glm;
use glm::{Mat3x4, Mat4, Vec3};
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
            m: m,
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
            mint: r.mint,
            maxt: r.maxt,
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

        return bb;
    }

    pub fn axis_offset(x: &Vec3, y: &Vec3, z: &Vec3, o: &Vec3) -> Transform {
        let m = Mat3x4::from_columns(&[*x, *y, *z, *o]);
        let mut m = m.insert_row(3, 0.);
        m[(3, 3)] = 1.0;
        Transform::new(m)
    }

    pub fn translate(t: &Vec3) -> Transform {
        Transform::new(Mat4::new_translation(&t))
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
        let mut t: Transform = Default::default();
        for sub_t in json.as_array().unwrap() {
            t = parse_transform(&sub_t) * t;
        }
        return t;
    }
    // single transform
    let kv = json.as_object().unwrap();

    let read_vector3 = |v: &Value| from_value::<Vec3>(v.clone()).unwrap();
    let read = |s: &str, default| kv.get(s).map_or(default, read_vector3);

    if kv.contains_key("from")
        || kv.contains_key("at")
        || kv.contains_key("to")
        || kv.contains_key("up")
    {
        let from = read("from", Vec3::z());
        let to = read("to", Vec3::zeros());
        let at = read("at", Vec3::zeros()) + to;
        let up = read("up", Vec3::y());

        let dir = glm::normalize(&(from - at));
        let left = glm::normalize(&glm::cross(&up, &dir));
        let new_up = glm::normalize(&glm::cross(&dir, &left));

        return Transform::axis_offset(&left, &new_up, &dir, &from);
    } else if kv.contains_key("o")
        || kv.contains_key("x")
        || kv.contains_key("y")
        || kv.contains_key("z")
    {
        let o = read("o", Vec3::zeros());
        let x = read("x", Vec3::x());
        let y = read("y", Vec3::y());
        let z = read("z", Vec3::z());

        return Transform::axis_offset(&x, &y, &z, &o);
    } else if kv.contains_key("translate") {
        let t = read("translate", Vec3::zeros());
        return Transform::translate(&t);
    } else if kv.contains_key("scale") {
        let scale = kv.get("scale").unwrap().clone();
        if scale.is_number() {
            let sn: f32 = from_value(scale).expect("could not load 'scale' number Transform");
            return Transform::new(Mat4::new_scaling(sn));
        }
        let sv: Vec3 = from_value(scale).expect("could not load 'scale' vector Transform");
        return Transform::new(Mat4::new_nonuniform_scaling(&sv));
    } else if kv.contains_key("axis") || kv.contains_key("angle") {
        let axis = read("axis", Vec3::x());
        let angle = kv
            .get("angle")
            .map_or(0.0, |v: &Value| from_value::<f32>(v.clone()).unwrap());

        let angle = deg2rad(angle);
        return Transform::new(Mat4::from_scaled_axis(axis * angle));
    } else if kv.contains_key("matrix") {
        unimplemented!();
    } else {
        panic!("Unrecognized 'transform' command:")
    }
}

pub fn read_transform(v: &Value) -> Transform {
    let m = v.as_object().expect("Should be a map");
    if m.contains_key("transform") {
        parse_transform(&v["transform"])
    } else {
        Default::default()
    }
}

#[cfg(test)]
mod tests {

    use crate::ray::Ray;
    use crate::transform::Transform;
    use nalgebra::{Matrix4, Vector3};

    extern crate approx;

    #[test]
    fn test_transforms() {
        println!(
            "\n{}{}{}",
            "--------------------------------------------------------\n",
            "PROGRAMMING ASSIGNMENT, PART4: Transforms              \n",
            "--------------------------------------------------------\n"
        );

        // Darts also provides you with a Transform class.
        // Transform is a helper class that helps you transform geometric primitives
        // correctly Internally, it keeps track of a transformation matrix and its
        // inverse

        // Let's create a random transformation matrix
        let transformation_matrix = Matrix4::new(
            -0.846852, 0.107965, -0.520755, 0.0, -0.492958, -0.526819, 0.692427, 0.0, -0.199586,
            0.843093, 0.499359, 0.0, -0.997497, 0.127171, -0.613392, 1.0,
        )
        .transpose();

        // Now that we have a matrix, we can create a transform from it:
        let transform = Transform::new(transformation_matrix);

        // Go to transform.h and implement all required methods there. If you
        // implement them correctly, the code below will work:

        // Let's create some random geometric objects...

        let vector = Vector3::new(-0.997497, 0.127171, -0.6133920);
        let point = Vector3::new(0.617481, 0.170019, -0.0402539);
        let normal = Vector3::new(-0.281208, 0.743764, 0.6064130);
        let ray = Ray::new(
            Vector3::new(-0.997497, 0.127171, -0.613392),
            Vector3::new(0.962222, 0.264941, -0.0627278),
        );

        println!("vector = {}.", vector);
        println!("point  = {}.", point);
        println!("normal = {}.", normal);
        println!("ray.o  = {};\nray.d  = {}.", ray.origin, ray.direction);

        // ...and let's transform them!
        // We can transform things simply by multiplying it with the transform.
        // Let's check if you did it correctly:
        let transformed_vector = transform.vector(&vector);
        let transformed_point = transform.point(&point);
        let transformed_normal = transform.normal(&normal);
        let transformed_ray = transform.ray(&ray);

        let correct_transformed_vector = Vector3::new(0.904467, -0.6918370, 0.301205);
        let correct_transformed_point = Vector3::new(-1.596190, 0.0703303, -0.837324);
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

        println!("The forward transform matrix is\n{}.", transform.m);
        println!("The inverse transform matrix is\n{}.", transform.m_inv);
    }
}
