use std::sync::Arc;

use image::ImageResult;
const PI: f32 = core::f32::consts::PI;

use crate::{
    objects::{
        instance::Instance,
        material::{MixedMaterial, LAMBERTIAN, MIRROR},
        quad::Quad,
        sphere::Sphere,
        texture::ConstColorTexture,
        Object,
    },
    quaternions::Quaternion,
    rotation::EulerAngles,
    vec3::{ray::Ray, vec3::Vec3},
    viewport::{
        camera::Camera,
        ray_color::{self, light_biased_ray_cast, light_biased_ray_color, ray_color},
        scene::Scene,
        Viewport,
    },
};

#[test]
fn normal_sphere_test() -> Result<(), image::ImageError> {
    let i = Instance::new(Arc::new([Arc::new(Sphere {
        origin: Vec3::ZERO,
        radius: 0.5,
        mat: LAMBERTIAN.clone(),
        texture: Arc::new(ConstColorTexture::new(
            Vec3 {
                x: 0.8,
                y: 0.8,
                z: 0.8,
            },
            Vec3::ZERO,
        )),
    })]));
    let s = Scene::new(vec![i.clone()], 0.001, 1000.0);
    let cam = Camera::new(
        1.5,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        70.0,
        0.0,
    );

    let vp = Viewport::new(
        cam,
        s,
        Arc::new(ray_color),
        300,
        200,
        25,
        10,
        Vec3 {
            x: 0.9,
            y: 0.9,
            z: 0.9,
        },
        1.0,
    );

    vp.render().save("test_out/normal_sphere_test.png")?;
    let aabb = i.get_aabb();
    print!(
        "Sphere aabb: x: {:?}, y: {:?}, z:{:?}",
        aabb.x, aabb.y, aabb.z
    );
    Ok(())
}

#[test]
fn lambertian_sphere_test() -> Result<(), image::ImageError> {
    let i = Instance::new(Arc::new([Arc::new(Sphere {
        origin: Vec3::ZERO,
        radius: 0.5,
        mat: LAMBERTIAN.clone(),
        texture: Arc::new(ConstColorTexture::new(
            Vec3 {
                x: 0.8,
                y: 0.4,
                z: 0.8,
            },
            Vec3::ZERO,
        )),
    })]));
    let s = Scene::new(vec![i.clone()], 0.001, 1000.0);
    let cam = Camera::new(
        1.5,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        70.0,
        0.0,
    );

    let vp = Viewport::new(
        cam,
        s,
        Arc::new(ray_color),
        300,
        200,
        25,
        10,
        Vec3 {
            x: 0.9,
            y: 0.9,
            z: 0.0,
        },
        2.0,
    );
    vp.render().save("test_out/lambertian_test.png")?;
    Ok(())
}

#[test]
fn ray_cast_test() -> ImageResult<()> {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;
    const SAMPLES: usize = 10;

    let lights: Arc<[Arc<dyn Object + Send + Sync>]> = Arc::new([
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
            LAMBERTIAN.to_owned(),
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
                Vec3::ZERO,
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
            LAMBERTIAN.to_owned(),
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
                    z: 0.1,
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
                Vec3::ZERO,
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
            LAMBERTIAN.to_owned(),
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
                Vec3::ZERO,
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
                Vec3::ZERO,
            )),
        )),
    ]));
    let scene = Scene::new(
        vec![quad_box, Instance::new(Arc::new([lights[1].clone()]))],
        0.001,
        1000.0,
    );
    let cam = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        Vec3::ZERO,
        Vec3::UP,
        Vec3::FORWARD,
        90.0,
        0.0,
    );

    let rc = Box::new(move |r: Ray, vp: Arc<Viewport>, _: usize| {
        light_biased_ray_cast(r, vp, 0, lights.clone())
    });

    let vp = Viewport::new(
        cam,
        scene,
        Arc::new(rc),
        WIDTH,
        HEIGHT,
        SAMPLES,
        10,
        Vec3::ZERO,
        2.0,
    );

    vp.render().save("test_out/ray_cast_test.png")
}

fn make_scene() -> (Scene, Arc<[Arc<dyn Object + Send + Sync>]>) {
    let lights: Arc<[Arc<dyn Object + Send + Sync>]> = Arc::new([
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
            LAMBERTIAN.to_owned(),
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
                Vec3::ZERO,
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
            LAMBERTIAN.to_owned(),
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
                    z: 0.1,
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
                Vec3::ZERO,
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
                Vec3::ZERO,
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
                Vec3::ZERO,
            )),
        )),
    ]));
    (
        Scene::new(
            vec![quad_box, Instance::new(Arc::new([lights[1].clone()]))],
            0.001,
            1000.0,
        ),
        lights,
    )
}

#[test]
fn light_biased_ray_color_test() -> ImageResult<()> {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;
    const SAMPLES: usize = 100;
    const DEPTH: usize = 9;
    const BIASED_WEIGHT: f32 = 100.;

    let (scene, lights) = make_scene();
    let cam = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        Vec3::ZERO,
        Vec3::UP,
        Vec3::FORWARD,
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

    let vp = Viewport::new(
        cam.clone(),
        scene.clone(),
        Arc::new(ray_cast),
        WIDTH,
        HEIGHT,
        SAMPLES,
        10,
        Vec3::ZERO,
        2.0,
    );
    let vp2 = Viewport::new(
        cam,
        scene,
        Arc::new(biased_ray_color),
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        Vec3::ZERO,
        2.0,
    );

    vp.render().save("test_out/ray_color_control.png")?;
    vp2.render().save("test_out/ray_color_test.png")
}

#[cfg(test)]
mod material_tests;

#[test]
pub fn rotation_test() -> ImageResult<()> {
    const WIDTH: usize = 30_000;
    const HEIGHT: usize = 30_000;
    const SAMPLES: usize = 25;
    const DEPTH: usize = 2;
    const BIASED_WEIGHT: f32 = 100.;
    let cam = Camera::new(
        WIDTH as f32 / HEIGHT as f32,
        Vec3::ZERO,
        Vec3::UP,
        Vec3::FORWARD,
        50.0,
        0.0,
    );
    let mut r_box = Instance::new_box(
        Vec3 {
            x: -1.,
            y: -1.,
            z: -1.,
        },
        Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        },
        Arc::new(ConstColorTexture::new(Vec3::WHITE * 0.6, Vec3::ZERO)),
        LAMBERTIAN.clone(),
    );
    r_box.rotate(Quaternion::new_from_axis(PI / 4., Vec3::UP));
    println!("{:?}", r_box.getr());
    r_box.translate(Vec3::FORWARD * 5. + Vec3::RIGHT);

    let scene = Scene::new(vec![r_box], 0.0001, 10000.);

    let vp = Viewport::new(
        cam.clone(),
        scene.clone(),
        Arc::new(ray_color),
        WIDTH,
        HEIGHT,
        SAMPLES,
        DEPTH,
        Vec3::WHITE * 0.6,
        1.0,
    );

    vp.render().save("test_out/rotation_test.png")
}
