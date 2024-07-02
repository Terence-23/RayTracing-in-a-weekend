use std::sync::Arc;

use image::ImageResult;

use crate::{
    objects::{
        instance::Instance,
        material::{MirrorGlass, MixedMaterial, LAMBERTIAN, MIRROR},
        quad::Quad,
        sphere::Sphere,
        texture::ConstColorTexture,
        Object,
    },
    vec3::{ray::Ray, vec3::Vec3},
    viewport::{
        camera::Camera,
        ray_color::{light_biased_ray_cast, light_biased_ray_color, ray_color},
        scene::Scene,
        Viewport,
    },
};

#[test]
fn mixed_material_test() -> ImageResult<()> {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;
    const SAMPLES: usize = 100;
    const DEPTH: usize = 9;
    const BIASED_WEIGHT: f32 = 100.;

    let lights: Arc<[Arc<dyn Object>]> = Arc::new([
        Arc::new(Quad::new(
            Vec3 {
                x: -0.10,
                y: -0.10,
                z: 4.5,
            },
            Vec3 {
                x: 0.2,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.2,
                z: 0.0,
            },
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 2.0,
                    y: 2.0,
                    z: 2.0,
                },
            )),
        )),
        Arc::new(Sphere {
            origin: Vec3 {
                x: -0.40,
                y: 0.0,
                z: 4.5,
            },
            radius: 0.2,
            mat: LAMBERTIAN.to_owned(),

            texture: Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 2.0,
                    z: 4.0,
                },
            )),
        }),
    ]);
    let quad_box = Instance::new(Arc::new([
        //Red
        Arc::new(Quad::new(
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
            Arc::new(MixedMaterial::new(3.)),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 0.2,
                    z: 0.2,
                },
                Vec3::zero(),
            )),
        )),
        //Light
        lights[0].clone(),
        //Green
        Arc::new(Quad::new(
            Vec3 {
                x: -2.0,
                y: -2.0,
                z: 6.0,
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
            Arc::new(MixedMaterial::new(1.)),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 0.2,
                    y: 1.0,
                    z: 0.2,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            )),
        )),
        //Blue
        Arc::new(Quad::new(
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
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    z: 1.0,
                    y: 0.2,
                    x: 0.2,
                },
                Vec3::zero(),
            )),
        )),
        //Orange
        Arc::new(Quad::new(
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
            MIRROR.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 0.5,
                    z: 0.,
                },
                Vec3::zero(),
            )),
        )),
        //Teal
        Arc::new(Quad::new(
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
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 0.2,
                    y: 0.8,
                    z: 0.8,
                },
                Vec3::zero(),
            )),
        )),
    ]));
    let (scene, lights) = (
        Scene::new(
            vec![quad_box, Instance::new(Arc::new([lights[1].clone()]))],
            0.001,
            1000.0,
        ),
        lights,
    );

    let cam = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        Vec3::zero(),
        Vec3::up(),
        Vec3::forward(), //(Vec3::forward() * 5. + Vec3::right()).unit(),
        90.0,
        0.0,
    );
    let lclone = lights.clone();
    let ray_cast = Box::new(move |r: Ray, vp: Arc<Viewport>, d: usize| {
        light_biased_ray_cast(r, vp, d, lclone.clone())
    });
    let biased_ray_color = Box::new(move |r: Ray, vp: Arc<Viewport>, d: usize| {
        light_biased_ray_color(r, vp, d, lights.clone(), BIASED_WEIGHT)
    });

    let bg_color = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let vp = Viewport::new(
        cam.clone(),
        scene.clone(),
        Box::leak(ray_cast),
        WIDTH,
        HEIGHT,
        SAMPLES,
        10,
        bg_color.clone(),
        2.0,
    );
    let vp2 = Viewport::new(
        cam.clone(),
        scene.clone(),
        Box::leak(biased_ray_color),
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        bg_color.clone(),
        2.0,
    );
    let vp3 = Viewport::new(
        cam,
        scene,
        &ray_color,
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        bg_color.clone(),
        2.0,
    );

    eprintln!("1:");
    vp.render().save("test_out/materials/mixed_ray_cast.png")?;
    eprintln!("2:");
    vp2.render()
        .save("test_out/materials/mixed_ray_color.png")?;
    eprintln!("3:");
    vp3.render()
        .save("test_out/materials/mixed_ray_color_old.png")
}

#[test]
fn glass_material_test() -> ImageResult<()> {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;
    const SAMPLES: usize = 100;
    const DEPTH: usize = 9;
    const BIASED_WEIGHT: f32 = 100.;

    let lights: Arc<[Arc<dyn Object>]> = Arc::new([
        Arc::new(Quad::new(
            Vec3 {
                x: -0.10,
                y: -0.10,
                z: 4.5,
            },
            Vec3 {
                x: 0.2,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.2,
                z: 0.0,
            },
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 2.0,
                    y: 2.0,
                    z: 2.0,
                },
            )),
        )),
        Arc::new(Sphere {
            origin: Vec3 {
                x: -0.40,
                y: 0.0,
                z: 4.5,
            },
            radius: 0.2,
            mat: LAMBERTIAN.to_owned(),

            texture: Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 2.0,
                    z: 4.0,
                },
            )),
        }),
    ]);
    let quad_box = Instance::new(Arc::new([
        //Red
        Arc::new(Quad::new(
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
            Arc::new(MixedMaterial::new(3.)),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 0.2,
                    z: 0.2,
                },
                Vec3::zero(),
            )),
        )),
        //Light
        lights[0].clone(),
        //Green
        Arc::new(Quad::new(
            Vec3 {
                x: -2.0,
                y: -2.0,
                z: 6.0,
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
            Arc::new(MixedMaterial::new(1.)),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 0.2,
                    y: 1.0,
                    z: 0.2,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            )),
        )),
        //Blue
        Arc::new(Quad::new(
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
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    z: 1.0,
                    y: 0.2,
                    x: 0.2,
                },
                Vec3::zero(),
            )),
        )),
        //Orange
        Arc::new(Quad::new(
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
            MIRROR.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 1.0,
                    y: 0.5,
                    z: 0.,
                },
                Vec3::zero(),
            )),
        )),
        //Teal
        Arc::new(Quad::new(
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
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 0.2,
                    y: 0.8,
                    z: 0.8,
                },
                Vec3::zero(),
            )),
        )),
    ]));
    let glass = Instance::new(Arc::new([Arc::new(Quad::new(
        Vec3 {
            x: -2.0,
            y: -2.0,
            z: 3.0,
        },
        Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 3.5,
            z: 0.0,
        },
        Arc::new(MirrorGlass { ir: 1.5 }),
        Vec3::zero(),
        Arc::new(ConstColorTexture::new(
            Vec3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
            Vec3::zero(),
        )),
    ))]));
    let (scene, lights) = (
        Scene::new(
            vec![
                quad_box,
                Instance::new(Arc::new([lights[1].clone()])),
                // glass,
            ],
            0.001,
            1000.0,
        ),
        lights,
    );

    let cam = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        Vec3::zero(),
        Vec3::up(),
        Vec3::forward(), //(Vec3::forward() * 5. + Vec3::right()).unit(),
        90.0,
        0.0,
    );
    let lclone = lights.clone();
    let ray_cast = Box::new(move |r: Ray, vp: Arc<Viewport>, d: usize| {
        light_biased_ray_cast(r, vp, d, lclone.clone())
    });
    let biased_ray_color = Box::new(move |r: Ray, vp: Arc<Viewport>, d: usize| {
        light_biased_ray_color(r, vp, d, lights.clone(), BIASED_WEIGHT)
    });

    let bg_color = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let vp = Viewport::new(
        cam.clone(),
        scene.clone(),
        Box::leak(ray_cast),
        WIDTH,
        HEIGHT,
        SAMPLES,
        10,
        bg_color.clone(),
        2.0,
    );
    let vp2 = Viewport::new(
        cam.clone(),
        scene.clone(),
        Box::leak(biased_ray_color),
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        bg_color.clone(),
        2.0,
    );
    let vp3 = Viewport::new(
        cam,
        scene,
        &ray_color,
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        bg_color.clone(),
        2.0,
    );

    eprintln!("1:");
    vp.render().save("test_out/materials/glass_ray_cast.png")?;
    eprintln!("2:");
    vp2.render()
        .save("test_out/materials/glass_ray_color.png")?;
    eprintln!("3:");
    vp3.render()
        .save("test_out/materials/glass_ray_color_old.png")
}
