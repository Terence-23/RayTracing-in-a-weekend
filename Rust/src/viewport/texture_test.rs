use super::*;
use crate::objects::materials::{FUZZY3_M, METALLIC_M, SCATTER_M};
use crate::texture::texture::{ImageTexture, PerlinNoise};
use crate::write_img::img_writer::write_img_f32;

fn ray_color_d(r: Ray, scene: &Scene, depth: usize) -> Rgb<f32> {
    // eprintln!("D: {}", depth);
    if depth < 1 {
        // dbg!("recursion end");
        return Rgb([0.0, 0.0, 0.0]);
    }
    let mint = 0.0001;
    let maxt = 1000.0;

    let hit = scene.aabb.collision_normal(r, mint, maxt);

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
        // add a bit of an offset to prevent clipping
        next.origin += hit.normal * mint;
        // println!("depth: {}", depth);
        return (Vec3::from_rgb(ray_color_d(next, scene, depth - 1)) * cm).to_rgb();
    }
    // if depth != 9{
    //     eprintln!("Sky, {}", depth);
    // }
    // if depth == 10{
    //     dbg!(r);
    // }
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
}

#[test]
fn default_test() {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();

    rt.block_on(async {
        let samples = 100;
        let spheres = vec![
            // Sphere::new_with_texture(Vec3 {x: -0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(SCATTER_M), ImageTexture:: from_path("assets/default.png").expect("image not found")),
            // Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(SCATTER_M)),
            Sphere::new_with_texture(
                Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: -0.0,
                },
                0.05,
                Some(Vec3::new(1.0, 1.0, 1.0)),
                Some(METALLIC_M),
                ImageTexture::from_path("assets/earthmap.jpg").expect("image not found"),
            ),
        ];
        let scene = Scene::new_sphere(spheres);
        let viewport = Viewport::new_from_res(
            2000,
            2000,
            samples,
            10,
            2.0,
            Some(7.0),
            None,
            Some(Vec3 {
                x: -1.0,
                y: 0.0,
                z: -0.0,
            }),
            None,
            Some("texture test".to_string()),
            None,
        );

        let img = async_render(Box::new(viewport), ray_color_d, Box::new(scene)).await;

        write_img_f32(&img, "out/texture_test.png".to_string());
    });
}
#[test]
fn reflection_test() {
    let samples = 100;
    let spheres = vec![
        // Sphere::new_with_texture(Vec3 {x: -0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(SCATTER_M), ImageTexture:: from_path("assets/default.png").expect("image not found")),
        Sphere::new(
            Vec3 {
                x: 0.520,
                y: 0.0,
                z: -1.0,
            },
            0.45,
            Some(Vec3::new(0.95, 0.95, 0.95)),
            Some(METALLIC_M),
        ),
        Sphere::new_with_texture(
            Vec3 {
                x: -1001.0,
                y: 0.0,
                z: 0.0,
            },
            1000.0,
            None,
            Some(METALLIC_M),
            ImageTexture::from_path("assets/earthmap.jpg").expect("image not found"),
        ),
    ];
    let scene = Scene::new_sphere(spheres);
    let viewport = Viewport::new_from_res(
        400,
        300,
        samples,
        10,
        2.0,
        Some(90.0),
        Some(Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }),
        None,
        None,
        Some("Reflection test".to_string()),
        None,
    );

    // let img =  async_render(Box::new(viewport), ray_color_d, Box::new(scene)).await;
    let img = viewport.render(&ray_color_d, &scene);

    write_img_f32(&img, "out/texture_reflection_test.png".to_string());
}

#[test]
fn noise_test() {
    let samples = 100;
    let spheres = vec![
        Sphere::new(
            Vec3 {
                x: -1.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(0.8, 0.8, 0.8)),
            Some(METALLIC_M),
        ),
        Sphere::new(
            Vec3 {
                x: 1.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(0.8, 0.6, 0.2)),
            Some(METALLIC_M),
        ),
        Sphere::new_with_texture(
            Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            100.0,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(SCATTER_M),
            ImageTexture::from_color_noise(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                }
                .to_rgb(),
                0.01,
            ),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(0.8, 0.8, 0.0)),
            Some(SCATTER_M),
        ),
    ];
    let scene = Scene::new_sphere(spheres);
    let viewport = Viewport::new_from_res(
        400,
        225,
        samples,
        10,
        2.0,
        None,
        None,
        None,
        None,
        Some("Noise test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene);

    write_img_f32(&img, "out/noise_test.png".to_string());
}

#[test]
fn perlin_test() {
    let noise = PerlinNoise::new();
    for x in -3..3 {
        dbg!((x as f32) as isize & 255);
        dbg!(noise.noise(Vec3 {
            x: x as f32,
            y: 0.0,
            z: 0.0
        }));
    }
}
