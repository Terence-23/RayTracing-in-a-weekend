mod write_img;
mod vec3;
mod objects;
mod viewport;


use image::Rgb;
use vec3::{ray::Ray, vec3::Vec3};
use objects::objects::{Sphere, NO_HIT, Object, materials::*};
use viewport::viewport::{Scene, Viewport};
use write_img::img_writer::write_img_f32;

use tokio;
use futures::prelude::*;



fn ray_color_d(r: Ray, scene: &Scene, depth:usize) -> Rgb<f32> {
    // eprintln!("D: {}", depth);
    if depth < 1{
        
        return Rgb([0.0, 0.0, 0.0])
    }
    let mint = 0.001; let maxt = 1000.0;
    
    let hit = {
        let mut min_hit = scene.spheres[0].collision_normal(r, mint, maxt);
        for i in scene.spheres[..].into_iter().map(|sp| sp.collision_normal(r, mint, maxt)){

            if i == NO_HIT{continue;}
            if min_hit == NO_HIT || min_hit > i {
                min_hit = i;
            }
        }
        min_hit
    };

    if hit != NO_HIT {
        // eprintln!("Hit: {:?}", hit );
        let cm = hit.col_mod;
        let front = if r.direction.dot(hit.normal) > 0.0 {
            false
        } else {
            true
        };
        let mut next = hit.mat.on_hit(hit, r);
        if next.direction.close_to_zero(){
            next.direction = if front {hit.normal} else {hit.normal * -1.0};
        }
        // println!("depth: {}", depth);
        return (Vec3::from_rgb(ray_color_d(next, scene, depth-1)) * cm).to_rgb();
    }
    // eprintln!("Sky");
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
}

fn main() {
    println!("Hello, world!");
    const SAMPLES: usize = 100;
    const DEPTH: usize = 100;
    let spheres  = vec!{
        Sphere::new(Vec3 {x: -0.8, y: 0.0, z: -1.0,}, 0.4, Some(Vec3::new(1.0, 0.6, 0.6)), Some(FUZZY3_M)),
        Sphere::new(Vec3 {x: 0.6, y: 0.0, z: -1.2,}, 0.3, Some(Vec3::new(0.5, 0.9, 0.9)), Some(METALLIC_M)),
        Sphere::new(Vec3 {x: 2.4, y: 0.0, z: -0.8,}, 1.4, Some(Vec3::new(0.9, 0.9, 0.9)), Some(METALLIC_M)),
        Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -0.7,}, 0.3, Some(Vec3::new(1.0, 1.0, 1.0)), Some(GLASS_M)),
        Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -0.7,}, 0.2, Some(Vec3::new(1.0, 1.0, 1.0)), Some(GLASSR_M)),
        Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(SCATTER_M)),
        Sphere::new(Vec3 {x: 0.0, y: -1000.9, z: -5.0,}, 1000.0, Some(Vec3::new(0.8, 0.5, 1.0)), Some(EMPTY_M)),
    };
    let scene = Scene{spheres: spheres};
    let viewport = Viewport::new_from_res(1920, 1080, SAMPLES, DEPTH, 2.0, None, None, None, None, Some("Rendering".to_string()), None);


    use std::time::Instant;
    
    {
        println!("Rendering {} samples", viewport.height as u128 * viewport.width as u128 * SAMPLES as u128);
        let before = Instant::now();
        
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let future = viewport::viewport::async_render(viewport.to_owned(), ray_color_d, scene.to_owned());
        let img = rt.block_on(future);
        write_img_f32(img, "render_async.png".to_string());
        
        println!("Finished\nElapsed time: {:?}", before.elapsed());
    }
    {
        println!("Rendering {} samples", viewport.height as u128 * viewport.width as u128 * SAMPLES as u128);
        let before = Instant::now();
        
        let img = viewport.render(&ray_color_d, scene.to_owned());
        write_img_f32(img, "render.png".to_string());
    
        println!("Finished\nElapsed time: {:?}", before.elapsed());
    }
}

