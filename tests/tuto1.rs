use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use serde_json::{json, Value};
use std::rc::Rc;

use rustrt::camera::PinholeCamera;
use rustrt::image2d::Image2d;
use rustrt::ray::Ray;
use rustrt::surfaces::surface_group::SurfaceGroup;
use rustrt::transform::Transform;
use rustrt::surfaces::surface::Surface;
use rustrt::surfaces::sphere::Sphere;
use rustrt::materials::factory::create_material;
use rustrt::utils::lerp;
use rustrt::surfaces::surface::HitInfo;
use rustrt::materials::lambertian::Lambertian;
use rustrt::materials::material::Material;

fn vec2color(dir: &Vector3<f32>) -> Vector3<f32> {
    0.5 * (dir.add_scalar(1.0))
}

fn ray2color(ray: &Ray) -> Vector3<f32> {
    return vec2color(&ray.direction.normalize());
}

fn intersection2color(r: &Ray, sphere: &Sphere) -> Vector3<f32> {
    if let Some(hit) = sphere.intersect(r) {
        return vec2color(&hit.sn.normalize());
    } else {
        vec2color(&r.direction.normalize())
    }
}

// Generate rays by hand
#[test]
fn test_manual_camera_image() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 1: Generating rays by hand \n",
        "--------------------------------------------------------\n"
    );

    // Setup the output image
    let mut ray_image = Image2d::new(200, 100);

    let camera_origin = Vector3::new(0., 0., 0.);
    let image_plane_width = 4.;
    let image_plane_height = 2.;

    // loop over all pixels and generate a ray
    for y in 0..ray_image.size_y {
        for x in 0..ray_image.size_x {
            // Fill in ray_origin so that the ray starts at
            // camera_origin, and fill in ray_direction so that
            // 1) the x component of the direction varies from -image_plane_width/2 for the left-most pixel to
            //    +image_plane_width/2 for the right-most pixel
            // 2) the y component of the direction varies from +image_plane_height/2 for the top-most pixel to
            //    -image_plane_height/2 for the bottom-most pixel
            // 3) the z component is -1
            //
            // Make sure to calculate the ray directions to go through the center of each pixel
            let ray_origin = camera_origin.clone();
            let x_ = lerp(
                -image_plane_width / 2.0,
                image_plane_width / 2.0,
                (x as f32) / (ray_image.size_x as f32),
            );
            let y_ = lerp(
                image_plane_height / 2.0,
                -image_plane_height / 2.0,
                (y as f32) / (ray_image.size_y as f32),
            );
            let ray_direction = Vector3::new(x_, y_, -1.0) - ray_origin;
            let ray = Ray::new(ray_origin, ray_direction);

            // Generate a visual color for the ray so we can debug our ray directions
            ray_image[(x, y)] = ray2color(&ray);
        }
    }

    let filename = "scenes/assignment1/01_manual_ray_image.png".to_string();
    println!("Saving ray image to {}....", filename);
    ray_image.save(filename);
}

#[test]
fn test_json() {
    // Darts also includes a C++ library (https://github.com/nlohmann/json)
    // for parsing and manipulating JSON data.
    //
    // JSON is a human-readible data interchange format for expressing
    // attribute-value pairs. You can read more about it here:
    //      https://en.wikipedia.org/wiki/JSON
    //      https://www.json.org/
    //
    // In darts, we will use it for two purposes:
    //  1) As a generic way to pass named parameters to functions
    //  2) As a way to specify and load text-based scene files

    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 2: passing data using JSON \n",
        "--------------------------------------------------------\n"
    );

    let f = 2.0;
    let s = "a text string".to_string();
    let c3f = Vector3::new(1.0, 0.25, 0.5);
    let v3f = Vector3::new(2.0, 3.0, 4.0);
    let v4f = Vector4::new(2.0, 3.0, 4.0, 5.0);
    let n3f = Vector3::new(2.0, 3.0, 4.0);
    println!(
        "Original darts data:\nf = {},\ns = {},\nc3f = {},\nv3f = {},\nv4f = {},\nn3f = {}.",
        f, s, c3f, v3f, v4f, n3f
    );

    // All the basic darts data-types can easily be stored in a JSON object
    let j = json!({
        "my float": f,
        "my string": s,
        "my color" : c3f,
        "my vector3": v3f,
        "my normal": n3f
    });

    println!("The JSON object contains: {}", j.to_string());

    // serde_json::fr
    // We can also read these structures back out of the JSON object
    let f2 = &j["my float"];
    let s2 = &j["my string"];
    let c3f2: Vector3<f32> =
        serde_json::from_value(j["my color"].clone()).expect("could not read 'my color'");
    let v3f2: Vector3<f32> =
        serde_json::from_value(j["my vector3"].clone()).expect("could not read 'my vector3'");
    let n3f2: Vector3<f32> =
        serde_json::from_value(j["my normal"].clone()).expect("could not read 'my normal'");

    println!(
        "Retrieved darts data:\nf2 = {},\ns2 = {},\nc3f2 = {},\nv3f2 = {},\nn3f2 = {}.",
        f2, s2, c3f2, v3f2, n3f2
    );
    // There is a bug in the code above, and c3f2 doesn't have the same
    // value as the original c3f. Fix it.

    // Now we will pass a json object in place of explicit parameters to
    // a function. Go to the function below and implement the TODO.
    let parameters = json!({"radius": 2.3});
    function_with_json_parameters(&parameters);
}

fn function_with_json_parameters(j: &Value) {
    // Many of the constructors for ray tracing in darts take a JSON object. This
    // allows us to have a uniform interface for creating these structures while
    // allowing the constructors to retrieve the necessary values from the JSON
    // object. This will simplify our code for writing a parser for reading
    // scene files from disk.

    // Sometimes we may want to make a parameter optional, and take on some
    // default value if it is not specified.
    // Unfortunately, checking for a missing parameter using e.g. j["radius"]
    // will throw an exception if the parameter doesn't exist (if j is const).
    // Instead, we can use j.value<type>("name", default_value) to extract it.
    // This is what the constructors to Camera, Sphere, Quad, and Materials do.

    // Replace the below two lines to extract the parameters radius (default=1.f),
    // and center (default={0,0,0}) from the JSON object j

    let radius: f32 = serde_json::from_value(j["radius"].clone()).unwrap_or(1.0);
    let center: Vector3<f32> =
        serde_json::from_value(j["center"].clone()).unwrap_or(Vector3::zeros());
    println!("The radius is: {}", radius);
    println!("The center is:\n{}", center);
}

// Next, we will generate the same image, but using the Camera class
#[test]
fn test_camera_class_image() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 3: Camera class generate_ray\n",
        "--------------------------------------------------------\n"
    );

    // Setup the output image
    let mut ray_image = Image2d::new(200, 100);

    // Set up a camera with some reasonable parameters, using JSON
    // Look in camera.h and implement the camera constructor

    let camera_data = r#"{
        "vfov": 90.0,
        "resolution": [200, 100],
        "fdist": 1.0
    }"#;

    let camera_map: Value = serde_json::from_str(camera_data).unwrap();

    let camera: PinholeCamera = PinholeCamera::new(&camera_map);

    // println!("{}, {}, {}, {}, {}", camera.size, camera.focal_distance, camera.resolution, camera.aperture_radius, camera.transform.m);

    // loop over all pixels and ask the camera to generate a ray
    for y in 0..ray_image.size_y {
        for x in 0..ray_image.size_x {
            // Look in camera.h|cpp and implement Camera::generate_ray

            // We add 0.5 to the pixel coordinate to center the ray within the pixel
            let ray = camera.generate_ray(&Vector2::new((x as f32) + 0.5, (y as f32) + 0.5));
            ray_image[(x, y)] = ray2color(&ray);
        }
    }

    let filename = "scenes/assignment1/01_camera_ray_image.png".to_string();
    println!("Saving ray image to {}....", filename);
    ray_image.save(filename);
}

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

    let vector_error = (correct_transformed_vector - transformed_vector)
        .abs()
        .max();
    let point_error = (correct_transformed_point - transformed_point).abs().max();
    let normal_error = (correct_transformed_normal - transformed_normal)
        .abs()
        .max();
    let ray_error = (correct_transformed_ray_position - transformed_ray.origin)
        .abs()
        .max()
        .max(
            (correct_transformed_ray_direction - transformed_ray.direction)
                .abs()
                .max(),
        );

    println!("The forward transform matrix is\n{}.", transform.m);
    println!("The inverse transform matrix is\n{}.", transform.m_inv);

    println!(
        "Result of transform*vector is:\n{}, and it should be:\n{}.",
        transformed_vector, correct_transformed_vector
    );
    assert!(vector_error < 1e-5);

    println!(
        "Result of transform*point is:\n{}, and it should be:\n{}.",
        transformed_point, correct_transformed_point
    );
    assert!(point_error < 1e-5);

    println!(
        "Result of transform*normal is:\n{}, and it should be:\n{}.",
        transformed_normal, correct_transformed_normal
    );
    assert!(normal_error < 1e-5);

    println!(
        "transform*ray: transformed_ray.o is:\n{}, and it should be:\n{}.",
        transformed_ray.origin, correct_transformed_ray_position
    );
    println!(
        "transform*ray: transformed_ray.d is:\n{}, and it should be:\n{}.",
        transformed_ray.direction, correct_transformed_ray_direction
    );
    assert!(ray_error < 1e-5);
}

#[test]
fn test_xformed_camera_image() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 5: Transformed camera      \n",
        "--------------------------------------------------------\n"
    );

    // Setup the output image
    let mut ray_image = Image2d::new(200, 100);

    // Set up a camera with some reasonable parameters
    // Look in camera.h and implement the camera constructor
    let camera_json = json!({
        "vfov": 90.,
        "resolution": [ray_image.size_x, ray_image.size_y],
        "fdist": 1.0,
        "transform":{
            "from": [5.0, 15.0, -25.0],
            "to": [0.0, 0.0, 0.0],
            "up": [0.0, 1.0, 0.0]
        }
    });
    let camera = PinholeCamera::new(&camera_json);
    println!("{:?}", camera);

    // Generate a ray for each pixel in the ray image
    for y in 0..ray_image.size_y {
        for x in 0..ray_image.size_x {
            // Look in camera.h|cpp and implement camera.generate_ray

            // Make sure to take the camera transform into account!

            // We add 0.5 to the pixel coordinate to center the ray within the pixel
            let ray = camera.generate_ray(&Vector2::new(x as f32 + 0.5, y as f32 + 0.5));
            ray_image[(x, y)] = ray2color(&ray);
        }
    }

    let filename = "scenes/assignment1/01_xformed_camera_ray_image.png".to_string();
    println!("Saving ray image to {}....", filename);
    ray_image.save(filename);
}

#[test]
fn test_ray_sphere_intersection() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 6: Ray-Sphere intersection \n",
        "--------------------------------------------------------\n"
    );

    // Go to sphere.cpp and implement Sphere::intersect

    // Let's check if your implementation was correct:
    let material: Rc<dyn Material> = Rc::new(Lambertian {
        albedo: Vector3::new(1.0, 1.0, 1.0),
    });
    // let material = Lambertian(json!{{"albedo", 1.}});
    let test_sphere = Sphere::new(1.0, Rc::clone(&material));

    println!("Testing untransformed sphere intersection");
    let test_ray = Ray::new(Vector3::new(-0.25, 0.5, 4.0), Vector3::new(0.0, 0.0, -1.0));
    // HitInfo hit;
    if let Some(hit) = test_sphere.intersect(&test_ray) {
        let correct_t = 3.170844;
        let correct_p = Vector3::new(-0.25, 0.5, 0.829156);
        let correct_n = Vector3::new(-0.25, 0.5, 0.829156);

        println!(
            "Hit sphere! Distance is:\n{}, and it should be:\n{}",
            hit.t, correct_t
        );
        println!(
            "Intersection point is:\n{}, and it should be:\n{}",
            hit.p, correct_p
        );
        println!(
            "Intersection normal is:\n{}, and it should be:\n{}",
            hit.sn, correct_n
        );

        let sphere_error = [
            (correct_p - hit.p).abs().max(),
            (correct_n - hit.sn).abs().max(),
            (correct_t - hit.t).abs(),
        ]
        .into_iter()
        .reduce(f32::max)
        .unwrap();
        assert!(sphere_error < 1e-5);
    } else {
        panic!("Sphere intersection incorrect! Should hit sphere");
    }

    // Now, let's check if you implemented sphere transforms correctly!
    let transform = Transform::axis_offset(
        &Vector3::new(2.0, 0.0, 0.0),  // x-axis
        &Vector3::new(0.0, 1.0, 0.0),  // y-axis
        &Vector3::new(0.0, 0.0, 0.5),  // z-axis
        &Vector3::new(0.0, 0.25, 5.0), // translation
    );
    let transformed_sphere = Sphere {
        radius: 1.0,
        transform: transform,
        material: Rc::clone(&material),
    };
    let test_ray = Ray::new(Vector3::new(1.0, 0.5, 8.0), Vector3::new(0.0, 0.0, -1.0));

    println!("Testing transformed sphere intersection");
    if let Some(hit) = transformed_sphere.intersect(&test_ray) {
        let correct_t = 2.585422;
        let correct_p = Vector3::new(1.0, 0.5, 5.41458);
        let correct_n = Vector3::new(0.147442, 0.147442, 0.978019);

        println!(
            "Hit sphere! Distance is:\n{}, and it should be:\n{}",
            hit.t, correct_t
        );
        println!(
            "Intersection point is:\n{}, and it should be:\n{}",
            hit.p, correct_p
        );
        println!(
            "Intersection normal is:\n{}, and it should be:\n{}",
            hit.sn, correct_n
        );

        let sphere_error = [
            (correct_p - hit.p).abs().max(),
            (correct_n - hit.sn).abs().max(),
            (correct_t - hit.t).abs(),
        ]
        .into_iter()
        .reduce(f32::max)
        .unwrap();
        assert!(sphere_error < 1e-5);
    } else {
        panic!("Transformed sphere intersection incorrect! Should hit sphere");
    }
}

/// Now: Let's allow our camera to be positioned and oriented using a Transform, and will use it to raytrace a Sphere
#[test]
fn test_sphere_image() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 7: False-color sphere image\n",
        "--------------------------------------------------------\n"
    );

    // Setup the output image
    let mut ray_image = Image2d::new(200, 100);

    // Set up a camera with some reasonable parameters
    let camera_json = json!({
        "vfov": 90.,
        "resolution": [ray_image.size_x, ray_image.size_y],
        "fdist": 1.0,
        "transform":{
            "from": [5.0, 15.0, -25.0],
            "to": [0.0, 0.0, 0.0],
            "up": [0.0, 1.0, 0.0]
        }
    });
    let camera = PinholeCamera::new(&camera_json);

    // letz   material = DartsFactory<Material>::create(json{{"type", "lambertian"}, {"albedo", 1.f}});
    // Sphere sphere(20.f, material);

    let material = create_material(json!({"type": "lambertian", "albedo": 1.0 }));
    let sphere = Sphere::new(20.0, material);

    // Generate a ray for each pixel in the ray image
    for y in 0..ray_image.size_y {
        for x in 0..ray_image.size_x {
            // TODO: Look in camera.h|.cpp and implement camera.generate_ray

            // Make sure to take the camera transform into account!

            // We add 0.5 to the pixel coordinate to center the ray within the
            // pixel
            // let ray = camera.generate_ray(Vec2f(x + 0.5f, y + 0.5f));

            // If we hit the sphere, output the sphere normal; otherwise,
            // convert the ray direction into a color so we can have some visual
            // debugging
            // ray_image(x, y) = intersection2color(ray, sphere);

            let ray = camera.generate_ray(&Vector2::new(x as f32 + 0.5, y as f32 + 0.5));
            ray_image[(x, y)] = intersection2color(&ray, &sphere);
        }
    }

    let filename = "scenes/assignment1/01_xformed_camera_sphere_image.png".to_string();
    println!("Saving ray image to {}....", filename);
    ray_image.save(filename);
}

#[test]
fn test_materials() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 8: Materials               \n",
        "--------------------------------------------------------\n"
    );

    // Look at material.h|cpp and then go implement the Lambertian and Metal materials in lambertian.cpp and
    // metal.cpp, respectively

    // Note the line at the end of both files that looks something like this:
    // DARTS_REGISTER_CLASS_IN_FACTORY(Material, Lambertian, "lambertian")
    // This macro creates some code that informs a "Factory" how to create Materials of type Lambertian, and allows us
    // to later create them by providing the key "lambertian".

    // Let's see how this works by creating a red Lambertian material.
    //
    // The DartsFactory<Material>::create() function accepts a json object as a parameter, and returns a shared_ptr to a
    // Material. It determines the specific Material class to use by looking for a "type" field in the json object. In
    // this case it is "lambertian", which matches the third parameter in the DARTS_REGISTER_CLASS_IN_FACTORY macro
    // above
    let surface_color = Vector3::new(1.0, 0.25, 0.25);

    let lambert_json = json!({
        "type": "lambertian",
        "albedo": surface_color
    });
    let lambert_material = create_material(lambert_json);

    // And now let's create a slightly shiny metal surface
    let metal_json = json!({
        "type": "metal",
        "albedo": surface_color,
        "roughness": 0.3
    });
    let metal_material = create_material(metal_json);

    // Later we will also use DartsFactory<Type>::create() with different classes for the Type, such as Surface,
    // Integrator, etc.

    // Let's create a fictitious hitpoint
    let surface_point = Vector3::new(1.0, 2.0, 0.0);
    let normal = Vector3::new(1.0, 2.0, -1.0).normalize();
    let hit = HitInfo {
        t: 0.0,
        p: surface_point,
        uv: Vector2::new(0.0, 0.0),
        gn: normal,
        sn: normal,
        mat: Rc::clone(&lambert_material),
    };

    // And a fictitious ray
    let ray = Ray::new(Vector3::new(2.0, 3.0, -1.0), Vector3::new(-1.0, -1.0, 1.0));

    // Now, let's test your implementation!
    if let Some((lambert_attenuation, lambert_scattered)) = lambert_material.scatter(&ray, &hit) {
        let correct_origin = surface_point.clone();
        let correct_attenuation = surface_color.clone();
        let correct_direction = Vector3::new(1.206627e+00, 3.683379e-01, -8.104229e-01);

        println!(
            "Scattered ray origin is:\n{}, and it should be:\n{}.",
            lambert_scattered.origin, correct_origin
        );
        println!(
            "Attenuation is:\n{}, and it should be:\n{}.",
            lambert_attenuation, correct_attenuation
        );
        println!(
            "Ray direction is:\n{}, and it should be:\n{}.",
            lambert_scattered.direction, correct_direction
        );

        // , (correct_direction - lambert_scattered.direction).abs().max()

        let lambert_error = [
            (correct_origin - lambert_scattered.origin).abs().max(),
            (lambert_attenuation - correct_attenuation).abs().max(),
        ]
        .into_iter()
        .reduce(f32::max)
        .unwrap();
        assert!(lambert_error < 1e-5, "lambert error is too big");
    } else {
        println!("Lambert scatter incorrect! Scattering should have been successful\n");
    }

    println!("Testing metal scatter");
    if let Some((metal_attenuation, metal_scattered)) = metal_material.scatter(&ray, &hit) {
        let correct_origin = surface_point.clone();
        let correct_attenuation = surface_color.clone();
        let correct_direction = Vector3::new(2.697650e-01, 9.322242e-01, -2.421507e-01);

        println!(
            "Scattered! Ray origin is:\n{}, and it should be:\n{}.",
            metal_scattered.origin, correct_origin
        );
        println!(
            "Attenuation is:\n{}, and it should be:\n{}.",
            metal_attenuation, correct_attenuation
        );
        println!(
            "Ray direction is:\n{}, and it should be:\n{}.",
            metal_scattered.direction, correct_direction
        );

        // , (correct_direction - metal_scattered.direction).abs().max()

        let metal_error = [
            (correct_origin - metal_scattered.origin).abs().max(),
            (metal_attenuation - correct_attenuation).abs().max(),
        ]
        .into_iter()
        .reduce(f32::max)
        .unwrap();
        assert!(metal_error < 1e-5, "lambert error is too big");
    } else {
        println!("Metal scatter incorrect! Scattering should have been successful\n");
    }
}

#[test]
fn test_recursive_raytracing() {
    println!(
        "\n{}{}{}",
        "--------------------------------------------------------\n",
        "PROGRAMMING ASSIGNMENT, PART 9: Recursive Ray Tracing   \n",
        "--------------------------------------------------------\n"
    );

    // let intersection_tests = RelaxedCounter::new(0);
    // let rays_traced = RelaxedCounter::new(0);

    // Setup the output image
    let mut ray_image = Image2d::new(300, 150);

    // We want to average over several rays to get a more pleasing result
    const NUM_SAMPLES: u32 = 4;

    // Set up a camera with some reasonable parameters
    let camera_json = json!({
        "vfov": 45.,
        "resolution": [ray_image.size_x, ray_image.size_y],
        "fdist": 1.0,
        "transform":{
            "from": [1.9, 0.8, -3.5],
            "to": [1.9, 0.8, 0.0],
            "up": [0.0, 1.0, 0.0]
        }
    });
    let camera = PinholeCamera::new(&camera_json);
    println!("{:?}", camera);

    let ground = create_material(json!({"type": "lambertian", "albedo": 0.5 }));
    let matte =
        create_material(json!({"type": "lambertian", "albedo": Vector3::new(1.0, 0.25, 0.25) }));
    let shiny = create_material(json!({"type": "metal", "albedo": 1.0, "roughness": 0.3}));

    let matte_sphere = Rc::new(Sphere {
        radius: 1.0,
        transform: Transform::translate(&Vector3::new(3., 1., 0.)),
        material: matte,
    });
    let shiny_sphere = Rc::new(Sphere {
        radius: 1.0,
        transform: Transform::translate(&Vector3::new(0., 1., 1.)),
        material: shiny,
    });
    let ground_sphere = Rc::new(Sphere {
        radius: 1000.0,
        transform: Transform::translate(&Vector3::new(0., -1000., 0.)),
        material: ground,
    });

    // To raytrace more than one object at a time, we can put them into a group
    let mut scene = SurfaceGroup::new();
    scene.add_child(matte_sphere);
    scene.add_child(shiny_sphere);
    scene.add_child(ground_sphere);

    {
        // let progress_bar = ProgressBar::new(ray_image.size() as u64);
        // progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        // .unwrap()
        // .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        // .progress_chars("#>-"));
        // Generate a ray for each pixel in the ray image
        for y in 0..ray_image.size_y {
            for x in 0..ray_image.size_x {
                let mut color = Vector3::new(0.0, 0.0, 0.0);
                for _ in 0..NUM_SAMPLES {
                    // rays_traced.inc(1);
                    let ray =
                        camera.generate_ray(&Vector2::new((x as f32) + 0.5, (y as f32) + 0.5));
                    // Call recursive_color ``num_samples'' times and average the
                    // results. Assign the average color to ``color''
                    color += recursive_color(&ray, &scene, 0) / (NUM_SAMPLES as f32);
                }
                ray_image[(x, y)] = color;
                // progress_bar.inc(1);
            }
        }
        // println!("Rendering time : {:?}", progress_bar.elapsed());
    } // progress reporter goes out of scope here

    // println!("Average number of intersection tests per ray: {} ", (intersection_tests as f32) / (rays_traced as f32));

    let filename = "scenes/assignment1/01_recursive_raytracing.png".to_string();
    println!("Saving rendered image to {} ...", filename);
    ray_image.save(filename);
}

fn recursive_color(ray: &Ray, scene: &SurfaceGroup, depth: u32) -> Vector3<f32> {
    const MAX_DEPTH: u32 = 4;
    const BLACK: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    const WHITE: Vector3<f32> = Vector3::new(1.0, 1.0, 1.0);

    // Implement this function
    // Pseudo-code:
    //
    // if scene.intersect:
    // 		if depth < max_depth and hit_material.scatter(....) is successful:
    //			recursive_color = call this function recursively with the scattered ray and increased depth
    //          return attenuation * recursive_color
    //		else
    //			return black;
    // else:
    // 		return white

    if let Some(hit) = scene.intersect(ray) {
        if depth < MAX_DEPTH {
            if let Some((attenuation, scattered)) = hit.mat.scatter(ray, &hit) {
                return attenuation.component_mul(&recursive_color(&scattered, &scene, depth + 1));
            }
            return BLACK;
        } else {
            return BLACK;
        }
    } else {
        return WHITE;
    }
}

fn main() {
    test_manual_camera_image();
    test_json();
    test_camera_class_image();

    test_transforms();
    test_xformed_camera_image();

    test_ray_sphere_intersection();
    test_sphere_image();

    test_materials();
    test_recursive_raytracing();
}
