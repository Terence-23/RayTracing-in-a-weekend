mod objects;
mod vec3;

#[allow(dead_code,unused_imports)]
mod viewport;
mod write_img;
mod texture;

use image::Rgb;
use objects::{materials::*, Object, Sphere, NO_HIT};
use vec3::{ray::Ray, vec3::Vec3};
use viewport::{Scene, Viewport, Img};
use write_img::img_writer::write_img_f32;

use tokio;

fn ray_color_d(r: Ray, scene: &Scene, depth: usize) -> Rgb<f32> {
    // eprintln!("D: {}", depth);
    if depth < 1 {
        return Rgb([0.0, 0.0, 0.0]);
    }
    let mint = 0.001;
    let maxt = 1000.0;

    // let hit = {
    //     let mut min_hit = scene.spheres[0].collision_normal(r, mint, maxt);
    //     for i in scene.spheres[..]
    //         .into_iter()
    //         .map(|sp| sp.collision_normal(r, mint, maxt))
    //     {
    //         if i == NO_HIT {
    //             continue;
    //         }
    //         if min_hit == NO_HIT || min_hit > i {
    //             min_hit = i;
    //         }
    //     }
    //     min_hit
    // };
    let hit = scene.aabb.collision_normal(r, mint, maxt);

    if hit != NO_HIT {
        // eprintln!("Hit: {:?}", hit );
        let cm = hit.col_mod;
        let front = if r.direction.dot(hit.normal) > 0.0 {
            false
        } else {
            true
        };
        let mut next = hit.mat.on_hit(hit, r);
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

#[allow(unused_imports)]
use std::{time::Instant, process::Command};

#[allow(unused)]
fn test_run(f_name:String, viewport: Viewport, ray_color: impl Fn(Ray, &Scene, usize) -> Rgb<f32> + std::marker::Send+ std::marker::Copy + 'static, scene: &Scene) -> Vec<Img>{
        println!(
            "Rendering {} samples",
            viewport.height as u128 * viewport.width as u128 * viewport.samples as u128 * viewport.number_of_frames as u128
        );
        let before = Instant::now();

        let rt = tokio::runtime::Runtime::new().unwrap();
        let future =
            viewport::render_multi(viewport.to_owned(), ray_color, scene.to_owned());
        let img = rt.block_on(future);
        write_img_f32(&img[0], f_name);

        println!("Finished\nElapsed time: {:?}, num of frames: {:?}", before.elapsed(), img.len());
        return img
    }

fn main() {
    println!("Hello, world!");
    const SAMPLES: usize = 100;
    const DEPTH: usize = 100;
    let spheres = vec![
        Sphere::new(
            Vec3 {
                x: -0.8,
                y: 0.0,
                z: -1.0,
            },
            0.4,
            Some(Vec3::new(1.0, 0.6, 0.6)),
            Some(FUZZY3_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.6,
                y: 0.0,
                z: -1.2,
            },
            0.3,
            Some(Vec3::new(0.5, 0.9, 0.9)),
            Some(METALLIC_M),
        ),
        Sphere::new_moving(
            Vec3 {
                x: 2.4,
                y: 0.0,
                z: -0.8,
            },
            1.4,
            Some(Vec3::new(0.9, 0.9, 0.9)),
            Some(METALLIC_M),
            Vec3 {
                x: 0.0,
                y: 60.0,
                z: 0.0,
            },
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -0.7,
            },
            0.3,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(GLASS_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -0.7,
            },
            0.2,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(GLASSR_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            1.0,
            Some(Vec3::new(0.5, 1.0, 0.0)),
            Some(SCATTER_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: -1000.9,
                z: -5.0,
            },
            1000.0,
            Some(Vec3::new(0.8, 0.5, 1.0)),
            Some(EMPTY_M),
        ),
    ];
    let _scene = Scene::new(spheres);
    let mut viewport = Viewport::new_from_res(
        4000,
        4000,
        SAMPLES,
        DEPTH,
        2.0,
        None,
        None,
        None,
        None,
        Some("Rendering".to_string()),
        None,
    );
    viewport.frame = 0;
    viewport.start_frame =0;
    viewport.fps = 60.0;
    viewport.number_of_frames = 1;
    viewport.shutter_speed = 0.0;

    let ltr_spheres = vec![
        Sphere::new_moving(Vec3 { x: -1.0, y: 0.0, z: -1.0 }, 0.4, Some(Vec3::new(0.9, 0.9, 0.9)), Some(METALLIC_M), Vec3::new(1.0, 0.0, 0.0)),
        Sphere::new_moving(Vec3 { x: 0.0, y: 1.0, z: -0.9 }, 0.5, Some(Vec3::new(1.0, 0.0, 1.0)), Some(SCATTER_M), Vec3::new(0.0, 0.0, 0.0)),
    ];

    let ltr_scene = Scene::new(ltr_spheres);

    viewport.fps = 15.0;
    viewport.number_of_frames = 1; //20;
    viewport.shutter_speed = 0.0;

    // write_img_f32(viewport.render(&ray_color_d, ltr_scene.clone()), "Default image.png".to_string());
    let _video = test_run( "First frame.png".to_string(), viewport, ray_color_d, &ltr_scene);
    
    // Command::new("mkdir").arg("-p").arg("/tmp/video").spawn().expect("Failed to execute mkdir");
    // for (i, img) in video.iter().enumerate(){
    //     write_img_f32(img.to_vec(), format!("/tmp/video/img{:0>5}.png", i))
    // }
    
    // match Command::new("rm").arg("video.mp4").spawn(){
    //     Ok(_) => println!("Removed video.mp4"),
    //     Err(e) => println!("Error: {}", e)
    // };
    // let mut command = Command::new("ffmpeg");
    // command.arg("-framerate").arg("10")
    // // .arg("-pattern_type").arg("glob")
    // .arg("-r").arg("5")
    // .arg("video.mp4");
    // command.arg("-i").arg("/tmp/video/img%05d.png");
    // // for i in in_files{
    // //     command.arg("-i").arg(i);
    // // }
    // println!("Command: {:?}", command);
    // command.spawn().expect("Failed to execute ffmpeg");

}
