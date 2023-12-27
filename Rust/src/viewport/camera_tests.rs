use super::*;
use crate::objects::materials::{METALLIC_M, SCATTER_M};
use crate::write_img::img_writer::write_img_f32;

const WIDTH: u64 = 400;
const HEIGHT: u64 = 300;
const SAMPLES: usize = 100;
const DEPTH: usize = 10;
const GAMMA: f32 = 2.0;

fn ray_color_d(r: Ray, scene: &Scene, depth: usize) -> Rgb<f32> {
    // eprintln!("D: {}", depth);
    if depth < 1 {
        return Rgb([0.0, 0.0, 0.0]);
    }
    let mint = 0.001;
    let maxt = 1000.0;

    let hit = {
        let mut min_hit = scene.spheres[0].collision_normal(r, mint, maxt);
        for i in scene.spheres[..]
            .into_iter()
            .map(|sp| sp.collision_normal(r, mint, maxt))
        {
            if i == None {
                continue;
            }
            if min_hit == None || min_hit > i {
                min_hit = i;
            }
        }
        min_hit
    };

    if let Some(hit) = hit {
        // eprintln!("Hit: {:?}", hit );
        let cm = hit.col_mod;
        let front = if r.direction.dot(hit.normal) > 0.0 {
            false
        } else {
            true
        };
        let (mut next, _) = hit.mat.on_hit(hit, r);
        if next.direction.close_to_zero() {
            next.direction = if front { hit.normal } else { hit.normal * -1.0 };
        }
        // println!("depth: {}", depth);
        return (Vec3::from_rgb(ray_color_d(next, scene, depth - 1)) * cm).to_rgb();
    }
    // eprintln!("Sky");
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
}

fn scene() -> Scene {
    Scene::new_sphere(vec![
        Sphere::new(
            Vec3 {
                x: -0.5,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(0.6, 0.6, 0.6)),
            Some(SCATTER_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.5,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(SCATTER_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            1.0,
            Some(Vec3::new(0.5, 1.0, 0.0)),
            Some(METALLIC_M),
        ),
    ])
}

#[test]
fn default_settings() {
    let viewport = Viewport::new_from_res(
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        GAMMA,
        None,
        None,
        None,
        None,
        Some("Camera: default test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene());

    write_img_f32(&img, "out/camera_default_test.png".to_string());
}
#[test]
fn fov_120() {
    let viewport = Viewport::new_from_res(
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        GAMMA,
        Some(120.0),
        None,
        None,
        None,
        Some("Camera: fov 120 test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene());

    write_img_f32(&img, "out/camera_fov_120_test.png".to_string());
}
#[test]
fn upside_down() {
    let viewport = Viewport::new_from_res(
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        GAMMA,
        None,
        None,
        None,
        Some(Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        }),
        Some("Camera: upside down test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene());

    write_img_f32(&img, "out/camera_upside_down_test.png".to_string());
}
#[test]
fn depth_of_field() {
    let viewport = Viewport::new_from_res(
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        GAMMA,
        None,
        None,
        None,
        Some(Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }),
        Some("Camera: depth of field test".to_string()),
        Some(0.015),
    );

    let img = viewport.render(&ray_color_d, &scene());

    write_img_f32(&img, "out/camera_depth_of_field_test.png".to_string());
}
