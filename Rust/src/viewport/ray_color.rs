use image::Rgb;

use crate::{
    objects::NO_HIT,
    vec3::{ray::Ray, vec3::Vec3},
};

use super::Scene;

pub fn ray_color_gradient(r: Ray, scene: &Scene, depth: usize) -> Rgb<f32> {
    // eprintln!("D: {}", depth);
    if depth < 1 {
        return Rgb([0.0, 0.0, 0.0]);
    }
    let mint = 0.001;
    let maxt = 100000.0;

    let hit = scene.collision_normal(r, mint, maxt);

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
        return (Vec3::from_rgb(ray_color_gradient(next, scene, depth - 1)) * cm).to_rgb();
    }
    // eprintln!("Sky");
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]);
}

pub fn ray_color_bg_color(r: Ray, scene: &Scene, depth: usize) -> Rgb<f32> {
    // eprintln!("D: {}", depth);
    if depth < 1 {
        return Rgb([0.0, 0.0, 0.0]);
    }
    let mint = 0.001;
    let maxt = 10000.0;

    let hit = scene.collision_normal(r, mint, maxt);

    if let Some(hit) = hit {
        // eprintln!("Hit: {:?}", hit );
        let cm = hit.col_mod;
        let front = if r.direction.dot(hit.normal) > 0.0 {
            false
        } else {
            true
        };
        let (mut next, emmited) = hit.mat.on_hit(hit, r);
        if next.direction.close_to_zero() {
            next.direction = if front { hit.normal } else { hit.normal * -1.0 };
        }
        // println!("depth: {}", depth);
        return (Vec3::from_rgb(ray_color_bg_color(next, scene, depth - 1)) * cm + emmited)
            .to_rgb();
    }
    // eprintln!("Sky");
    return scene.background_color.to_rgb();
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::{
        objects::{
            materials::{Material, SCATTER_M},
            quad::Quad,
            sphere::Sphere,
        },
        texture::texture::ImageTexture,
        vec3::vec3::Vec3,
        viewport::{async_render, ray_color::ray_color_bg_color, Scene, Viewport},
        write_img::img_writer::write_img_f32,
    };

    #[test]
    fn light_test() {
        let samples = 100;
        let _spheres = vec![Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.45,
            Some(Vec3::new(0.95, 0.95, 0.95)),
            Some(SCATTER_M),
        )];
        let quads = vec![
            //Red
            Quad::new(
                Vec3 {
                    x: -3.0,
                    y: -2.0,
                    z: 5.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [1.0, 0.2, 0.2] }),
            ),
            //Green
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                Material::new_emmiting(
                    0.0,
                    0.0,
                    1.0,
                    Vec3 {
                        x: 4.0,
                        y: 2.0,
                        z: 4.0,
                    },
                ),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 1.0, 0.2] }),
            ),
            //Blue
            Quad::new(
                Vec3 {
                    x: 3.0,
                    y: -2.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 0.2, 1.0] }),
            ),
            //Orange
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: 3.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [1.0, 0.5, 0.0] }),
            ),
            //Teal
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -3.0,
                    z: 5.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 0.8, 0.8] }),
            ),
        ];
        // let scene = Scene::new(spheres, quads.to_owned());
        let scene = Scene::new_quad(quads);
        let viewport = Viewport::new_from_res(
            400,
            400,
            samples,
            10,
            2.0,
            Some(80.0),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 9.0,
            }),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }),
            None,
            Some("Quad test".to_string()),
            None,
        );
        eprintln!("Running");

        let img = viewport.render(&ray_color_bg_color, &scene);

        write_img_f32(&img, "out/light_test.png".to_string());
    }
    #[test]
    fn box_test() {
        let samples = 100;
        let spheres = vec![Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            0.4,
            Some(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            Some(Material::new_emmiting(
                1.0,
                0.0,
                0.0,
                Vec3 {
                    x: 2.0,
                    y: 0.5,
                    z: 2.0,
                },
            )),
        )];

        let quads = vec![
            //Red
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 5.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [1.0, 0.2, 0.2] }),
            ),
            //Light
            // Quad::new(
            //     Vec3 {
            //         x: -0.5,
            //         y: -0.5,
            //         z: 1.001,
            //     },
            //     Vec3 {
            //         x: 1.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     Vec3 {
            //         x: 0.0,
            //         y: 1.0,
            //         z: 0.0,
            //     },
            //     Material::new_emmiting(
            //         0.0,
            //         0.0,
            //         1.0,
            //         Vec3 {
            //             x: 4.0,
            //             y: 2.0,
            //             z: 4.0,
            //         },
            //     ),
            //     Vec3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     ImageTexture::from_color(Rgb { 0: [1.0, 1.0, 1.0] }),
            // ),
            //Blue
            Quad::new(
                Vec3 {
                    x: 2.0,
                    y: -2.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 0.2, 1.0] }),
            ),
            //Orange
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: 2.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [1.0, 0.5, 0.0] }),
            ),
            //Teal
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 5.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 0.8, 0.8] }),
            ),
            //Green
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                Material::new_emmiting(
                    0.0,
                    0.0,
                    1.0,
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 1.0, 0.2] }),
            ),
        ];
        // let scene = Scene::new(spheres, quads.to_owned());
        let scene = Scene::new(spheres, quads, vec![]);
        let viewport = Viewport::new_from_res(
            400,
            400,
            samples,
            10,
            2.0,
            Some(90.0),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 4.0,
            }),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }),
            None,
            Some("Quad test".to_string()),
            None,
        );
        eprintln!("Running");

        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        let img = runtime.block_on(async_render(
            Box::new(viewport),
            &ray_color_bg_color,
            Box::new(scene),
        ));

        write_img_f32(&img, "out/light_box_test.png".to_string());
    }
}
