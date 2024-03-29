use nalgebra_glm::{length, lerp, Vec3};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde_json::Value;
use serde_json::{self, json};

pub fn create_example_scene(scene_number: i32) -> Value {
    match scene_number {
        0 => create_sphere_scene(),
        1 => create_sphere_plane_scene(),
        2 => create_steinbach_scene(),
        3 => create_shirley_scene(),
        _ => panic!(
            "Valid exemples scenes are 0-1-2-3, scene number {scene_number} is not implemented"
        ),
    }
}

fn create_sphere_scene() -> Value {
    json!(
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
    })
}

fn create_sphere_plane_scene() -> Value {
    json!(
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
        "background": [1, 1, 1],
        "accelerator" : { "type": "bbh"}
    })
}

fn create_steinbach_scene() -> Value {
    // Compose the camera
    let mut scene = json!({
        "camera":{
            "transform":{
                "from": [-10.0, 10.0, 40.0],
                "to": [0.0, -1.0, 0.0],
                "up": [0.0, 1.0, 0.0]
            },
            "vfov" : 18.,
            "resolution": [512, 512]
        },
        "sampler": {"samples": 10},
        "background": [1, 1, 1],
        "accelerator" : { "type": "bbh"}
    });

    let object_center = Vec3::new(0.0, 0.0, 0.0);
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

            let center = Vec3::new(-u * v.cos(), v * u.cos() * 0.75, u * v.sin());
            let kd = 0.35
                * lerp(
                    &lerp(&Vec3::new(0.9, 0.0, 0.0), &Vec3::new(0.0, 0.9, 0.0), t),
                    &lerp(&Vec3::new(0.0, 0.0, 0.9), &Vec3::new(0.0, 0.0, 0.0), t),
                    s,
                );

            surfaces.push(json!({
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
            }));
        }
    }

    surfaces.push(json!({
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
    }));
    scene["surfaces"] = Value::Array(surfaces);

    scene
}

fn create_shirley_scene() -> Value {
    let mut rng = ChaCha8Rng::seed_from_u64(420);

    // Compose the camera
    let mut scene = json!({
        "camera": {
            "transform":{
                "from":[13, 2, 3],
                "to": [0, 0, 0],
                "up": [0, 1, 0]
            },
            "vfov": 20,
            "fdist": 10,
            "aperture": 0.1,
            "resolution": [600, 400]
        },
        "sampler": {"samples": 10},
        "background": [1, 1, 1],
        "accelerator" : { "type": "bbh"}
    });

    let mut surfaces: Vec<Value> = Vec::new();

    // ground plane
    surfaces.push(json!({
        "type": "quad",
        "size": [100, 100],
        "transform":{
            "o": [0.0, 0.0, 0.0],
            "x": [1.0, 0.0, 0.0],
            "y": [0.0, 0.0, -1.0],
            "z": [0.0, 1.0, 0.0]},
         "material": {
            "type": "lambertian",
            "albedo": [0.5, 0.5, 0.5]
        }
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );
            if length(&(center - Vec3::new(4.0, 0.2, 0.0))) > 0.9 {
                let mut sphere =
                    json!({"type": "sphere", "radius": 0.2, "transform": {"translate": center}});

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    );
                    sphere["material"] = json!({"type": "lambertian", "albedo": albedo});
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new(
                        0.5 * (1.0 + rng.gen::<f32>()),
                        0.5 * (1.0 + rng.gen::<f32>()),
                        0.5 * (1.0 + rng.gen::<f32>()),
                    );
                    let rough = 0.5 * rng.gen::<f32>();
                    sphere["material"] =
                        json!({"type": "metal", "albedo": albedo, "roughness": rough});
                } else {
                    // glass
                    sphere["material"] = json!({"type": "dielectric", "ior": 1.5});
                }
                surfaces.push(sphere);
            }
        }
    }

    surfaces.push(json!({
        "type": "sphere",
        "radius": 1.0,
        "transform": {
            "translate": [0, 1, 0]
        },
        "material": {
            "type": "dielectric",
            "ior": 1.5
        }
    }));
    surfaces.push(json!({
        "type": "sphere",
        "radius": 1.0,
        "transform": {
            "translate": [-4, 1, 0]
        },
        "material": {
            "type": "lambertian",
            "albedo": [0.4, 0.2, 0.1]
        }
    }));
    surfaces.push(json!({
        "type": "sphere",
        "radius": 1.0,
        "transform": {
            "translate": [4, 1, 0]
        },
        "material": {
            "type": "metal",
            "albedo": [0.7, 0.6, 0.5],
            "roughness": 0.0
        }
    }));

    scene["surfaces"] = Value::Array(surfaces);

    scene
}
