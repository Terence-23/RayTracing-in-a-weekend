use super::*;
use crate::objects::materials::{EMPTY_M, FUZZY3_M, GLASSR_M, GLASS_M, METALLIC_M, SCATTER_M};
use crate::write_img::img_writer::write_img_f32;

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
        let (mut next, _, _) = hit.mat.on_hit(hit, r);
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

#[test]
pub fn diffuse_test() {
    let samples = 100;
    let spheres = vec![
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
            Some(SCATTER_M),
        ),
    ];
    let scene = Scene::new_sphere(spheres);
    let viewport = Viewport::new_from_res(
        800,
        600,
        samples,
        10,
        2.0,
        None,
        None,
        None,
        None,
        Some("Diffuse test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene);

    write_img_f32(&img, "out/diffuse_test.png".to_string());
}
#[test]
pub fn metal_test() {
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
            Some(FUZZY3_M),
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
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(0.7, 0.30, 0.30)),
            Some(SCATTER_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            100.0,
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
        Some("Metallic test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene);

    write_img_f32(&img, "out/metal_test.png".to_string());
}
#[test]
pub fn glass_test_controll() {
    let samples = 100;
    let spheres = vec![
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(METALLIC_M),
        ),
        // Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.5, 0.9, 0.9)), Some(METALLIC_M)),
        // Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(SCATTER_M)),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            100.0,
            Some(Vec3::new(0.8, 0.5, 1.0)),
            Some(EMPTY_M),
        ),
    ];
    let scene = Scene::new_sphere(spheres);
    let viewport = Viewport::new_from_res(
        300,
        200,
        samples,
        10,
        2.0,
        None,
        None,
        None,
        None,
        Some("Control for dielectric test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene);

    write_img_f32(&img, "out/glass_test_c.png".to_string());
}
#[test]
pub fn glass_test() {
    let samples = 100;
    let spheres = vec![
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(GLASS_M),
        ),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.35,
            Some(Vec3::new(1.0, 1.0, 1.0)),
            Some(GLASSR_M),
        ),
        // Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.5, 0.9, 0.9)), Some(METALLIC_M)),
        // Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(SCATTER_M)),
        Sphere::new(
            Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            100.0,
            Some(Vec3::new(0.8, 0.5, 1.0)),
            Some(EMPTY_M),
        ),
    ];
    let scene = Scene::new_sphere(spheres);
    let viewport = Viewport::new_from_res(
        300,
        200,
        samples,
        10,
        2.0,
        None,
        None,
        None,
        None,
        Some("Dielectric test".to_string()),
        None,
    );

    let img = viewport.render(&ray_color_d, &scene);

    write_img_f32(&img, "out/glass_test.png".to_string());
}
