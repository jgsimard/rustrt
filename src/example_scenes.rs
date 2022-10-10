use nalgebra::Vector3;
use serde_json::Value;
use serde_json::{self, json};

extern crate nalgebra_glm as glm;

fn create_sphere_scene() -> Value {
    let data = r#"
    {
        "camera":
        {
            "transform": { "o": [0,0,4] },
            "resolution": [ 512, 512 ],
            "vfov": 45
        },
        "surfaces": [
            {
                "type": "sphere",
                "material": { "type": "lambertian", "albedo": [0.6,0.4,0.4] }
            }
        ],
        "sampler": {"samples": 1},
        "background": [1, 1, 1]
    }
    "#;
    serde_json::from_str(data).unwrap()
}

fn create_sphere_plane_scene() -> Value {
    let data = r#"
    {
        "camera":
        {
            "transform": { "o": [0,0,4] },
            "resolution": [ 512, 512 ],
            "vfov": 45
        },
        "surfaces": [
            {
                "type": "sphere",
                "radius": 1,
                "material": { "type": "lambertian", "albedo": [0.6,0.4,0.4] }
            },
            {
                "type": "quad",
                "transform": { "o": [0,-1,0], "x": [1,0,0], "y": [0,0,-1], "z": [0,1,0] },
                "size": [ 100, 100 ],
                "material": { "type": "lambertian", "albedo": [1,1,1] }
            }
        ],
        "sampler": {"samples": 1},
        "background": [1, 1, 1]
    }"#;
    return serde_json::from_str(data).unwrap();
}

pub fn create_example_scene(scene_number: i32) -> Value {
    match scene_number {
        0 => create_sphere_scene(),
        1 => create_sphere_plane_scene(),
        2 => create_steinbach_scene(),
        // 3 => create_shirley_scene(),
        _ => unimplemented!(),
    }
}

fn create_steinbach_scene() -> Value {
    // Compose the camera
    let mut j = json!({
        "camera":{
            "transform":{
                "from": [-10.0, 10.0, 40.0],
                "to": [0.0, -1.0, 0.0],
                "up": [0.0, 1.0, 0.0]
            },
            "vfov" : 18.,
            "resolution": [512, 512]
        }
    });

    // compose the image properties
    j["sampler"] = json!({"samples": 10});
    j["background"] = serde_json::to_value([1.0, 1.0, 1.0]).unwrap(); // json!({[1.0, 1.0 , 1.0]});

    let object_center = Vector3::new(0.0, 0.0, 0.0);
    let radius = 0.5;
    let num_s = 40;
    let num_t = 40;
    let mut surfaces: Vec<Value> = Vec::new();

    for is in 0..num_s {
        for it in 0..num_t {
            let s = (is as f32 + 0.5) / (num_s as f32);
            let t = (it as f32 + 0.5) / (num_t as f32);
            let u = s * 8.0 - 4.0;
            let v = t * 6.25;

            let center = Vector3::new(-u * v.cos(), v * u.cos() * 0.75, u * v.sin());
            let kd = 0.35
                * glm::lerp(
                    &glm::lerp(
                        &Vector3::new(0.9, 0.0, 0.0),
                        &Vector3::new(0.0, 0.9, 0.0),
                        t,
                    ),
                    &glm::lerp(
                        &Vector3::new(0.0, 0.0, 0.9),
                        &Vector3::new(0.0, 0.0, 0.0),
                        t,
                    ),
                    s,
                );

            let s = json!({
                "type": "sphere",
                "radius": radius,
                "transform":{
                    "o": object_center + center,
                    "x": [1.0, 0.0, 0.0],
                    "y": [0.0, 1.0, 0.0],
                    "z": [0.0, 0.0, 1.0]
                },
                "material": {
                    "type": "lambertian",
                    "albedo": kd
                }
            });
            surfaces.push(s);
        }
    }

    let s = json!({
        "type": "quad",
        "size": [100, 100],
        "transform":{
            "o": [0.0, -5.0, 0.0],
            "x": [1.0, 0.0, 0.0],
            "y": [0.0, 0.0, -1.0],
            "z": [0.0, 1.0, 0.0]
        },
        "material": {
            "type": "lambertian",
            "albedo": 1.0
        }
    });
    surfaces.push(s);

    j["surfaces"] = serde_json::Value::Array(surfaces);

    // // BVH
    // j["accelerator"] = {{"type", "bbh"}};

    return j;
}

// json create_shirley_scene()
// {
//     pcg32 rng = pcg32();

//     json j;

//     // Compose the camera
//     j["camera"] = {{"transform", {{"from", {13, 2, 3}}, {"to", {0, 0, 0}}, {"up", {0, 1, 0}}}},
//                    {"vfov", 20},
//                    {"fdist", 10},
//                    {"aperture", 0.1},
//                    {"resolution", {600, 400}}};

//     // compose the image properties
//     j["sampler"]["samples"] = 10;
//     j["background"]         = {1, 1, 1};

//     // BVH
//     j["accelerator"] = {{"type", "bbh"}};

//     // ground plane
//     j["surfaces"] +=
//         {{"type", "quad"},
//          {"size", {100, 100}},
//          {"transform",
//           {{"o", {0.0, 0.0, 0.0}}, {"x", {1.0, 0.0, 0.0}}, {"y", {0.0, 0.0, -1.0}}, {"z", {0.0, 1.0, 0.0}}}},
//          {"material", {{"type", "lambertian"}, {"albedo", {0.5, 0.5, 0.5}}}}};

//     for (int a = -11; a < 11; a++)
//     {
//         for (int b = -11; b < 11; b++)
//         {
//             float choose_mat = rng.nextFloat();
//             float r1 = rng.nextFloat();
//             float r2 = rng.nextFloat();
//             Vec3f center(a + 0.9f * r1, 0.2f, b + 0.9f * r2);
//             if (length(center - Vec3f(4.0f, 0.2f, 0.0f)) > 0.9f)
//             {
//                 json sphere = {{"type", "sphere"}, {"radius", 0.2f}, {"transform", {{"translate", center}}}};

//                 if (choose_mat < 0.8)
//                 { // diffuse
//                     float r1 = rng.nextFloat();
//                     float r2 = rng.nextFloat();
//                     float r3 = rng.nextFloat();
//                     float r4 = rng.nextFloat();
//                     float r5 = rng.nextFloat();
//                     float r6 = rng.nextFloat();
//                     Color3f albedo(r1*r2, r3*r4, r5*r6);
//                     sphere["material"] = {{"type", "lambertian"}, {"albedo", albedo}};
//                 }
//                 else if (choose_mat < 0.95)
//                 { // metal
//                     float r1 = rng.nextFloat();
//                     float r2 = rng.nextFloat();
//                     float r3 = rng.nextFloat();
//                     float r4 = rng.nextFloat();
//                     Color3f albedo(0.5f * (1 + r1), 0.5f * (1.0f + r2), 0.5f * (1.0f + r3));
//                     float   rough      = 0.5f * r4;
//                     sphere["material"] = {{"type", "metal"}, {"albedo", albedo}, {"roughness", rough}};
//                 }
//                 else
//                 { // glass
//                     sphere["material"] = {{"type", "dielectric"}, {"ior", 1.5}};
//                 }

//                 j["surfaces"] += sphere;
//             }
//         }
//     }

//     j["surfaces"] += {{"type", "sphere"},
//                       {"radius", 1.f},
//                       {"transform", {{"translate", {0, 1, 0}}}},
//                       {"material", {{"type", "dielectric"}, {"ior", 1.5}}}};
//     j["surfaces"] += {{"type", "sphere"},
//                       {"radius", 1.f},
//                       {"transform", {{"translate", {-4, 1, 0}}}},
//                       {"material", {{"type", "lambertian"}, {"albedo", {0.4, 0.2, 0.1}}}}};
//     j["surfaces"] += {{"type", "sphere"},
//                       {"radius", 1.f},
//                       {"transform", {{"translate", {4, 1, 0}}}},
//                       {"material", {{"type", "metal"}, {"albedo", {0.7, 0.6, 0.5}}, {"roughness", 0.0}}}};

//     return j;
// }
