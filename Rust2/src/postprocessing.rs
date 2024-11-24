use std::cmp::min;

use image::{ImageBuffer, Rgb};

use crate::{
    vec3::ray::Ray,
    viewport::{scene::Scene, Viewport},
};

type Img = ImageBuffer<Rgb<u8>, Vec<u8>>;

pub(crate) enum ProximityType {
    Square,
    Edges,
}

pub(crate) struct Proximity {
    size: u32,
    _type: ProximityType,
}

impl Proximity {
    pub fn new(size: u32, proximity_type: ProximityType) -> Self {
        Self {
            size,
            _type: proximity_type,
        }
    }

    pub fn get_pixels(&self, x: u32, y: u32, width: u32, height: u32) -> Vec<(u32, u32)> {
        let left = x - min(x, self.size);
        let up = y - min(y, self.size);
        let right = x + min(width - x - 1, self.size);
        let down = y + min(height - y - 1, self.size);

        // println!(
        //     "get_pixels: l: {}, u: {}, r: {}, d: {}",
        //     left, up, right, down
        // );

        let mut pixels = vec![];

        for i in left..right {
            for j in up..down {
                match self._type {
                    ProximityType::Square => {
                        pixels.push((i, j));
                    }
                    ProximityType::Edges => {
                        if (if x < i { i - x } else { x - i } + if y < j { j - y } else { y - j }
                            < self.size)
                        {
                            pixels.push((i, j));
                        }
                    }
                }
            }
        }

        pixels
    }
}

fn intensity(p: &Rgb<u8>) -> f32 {
    0.2989 * (p.0[0] as f32) / 255.0
        + 0.5870 * (p.0[1] as f32) / 255.0
        + 0.1140 * (p.0[2] as f32) / 255.0
}

pub(crate) fn bilateral_filter(img: &Img, proximity: Proximity) -> Img {
    let (w, h) = img.dimensions();

    let spatial = (0.02 * ((w * w + h * h) as f32).sqrt()).ceil();

    let mut gradient_sum = 0.0;
    for y in 1..(h - 1) {
        for x in 1..(w - 1) {
            let Iu = intensity(img.get_pixel(x, y - 1));
            let Id = intensity(img.get_pixel(x, y + 1));
            let Ir = intensity(img.get_pixel(x + 1, y));
            let Il = intensity(img.get_pixel(x - 1, y));

            gradient_sum += ((Iu - Id) * (Iu - Id) + (Il - Ir) * (Il - Ir)).sqrt();
        }
    }

    let avg_gradient = gradient_sum / (((w - 2) * (h - 2)) as f32);

    let inv_range = 0.5 / (avg_gradient * avg_gradient);
    let inv_spatial = 0.5 / (spatial * spatial);

    // println!("range: {}, spatial: {}", avg_gradient, spatial);

    let n_img = ImageBuffer::from_fn(w, h, |x, y| {
        let mut w_sum = [0.0, 0.0, 0.0];
        let mut col_sum = [0.0; 3];
        let p = img.get_pixel(x, y);

        let pixels: Vec<_> = proximity.get_pixels(x, y, w, h);
        // println!("pixels: {:?}", pixels);

        for (xi, yi) in proximity.get_pixels(x, y, w, h) {
            let pi = img.get_pixel(xi, yi);
            // println!("pos: {} {}", xi, yi);
            for i in 0..3 {
                // print!("{} ", i);
                let w = (-inv_spatial
                    * ((xi as i64 - x as i64) * (xi as i64 - x as i64)
                        + (yi as i64 - y as i64) * (yi as i64 - y as i64))
                        as f32
                    - inv_range * ((pi.0[i] as i32 - p.0[i] as i32) as f32 / 255.0).powi(2))
                .exp();
                col_sum[i] += pi.0[i] as f32 * w / 255.0;
                w_sum[i] += w;
            }
            // println!("");
        }
        // println!(
        //     "x: {}, y: {}, pcol: {:?} \nw: {:?}, col: {:?}",
        //     x, y, p.0, w_sum, col_sum
        // );

        Rgb::<u8>([
            (col_sum[0] * 255.0 / w_sum[0]) as u8,
            (col_sum[1] * 255.0 / w_sum[1]) as u8,
            (col_sum[2] * 255.0 / w_sum[2]) as u8,
        ])
    });

    return n_img;
}

#[cfg(test)]
mod test {
    use std::{sync::Arc, time::Instant};

    use image::ImageResult;

    use crate::{
        objects::{
            instance::Instance,
            material::{LAMBERTIAN, MIRROR},
            quad::Quad,
            sphere::Sphere,
            texture::ConstColorTexture,
            Object,
        },
        postprocessing::{bilateral_filter, Proximity, ProximityType},
        vec3::{ray::Ray, vec3::Vec3},
        viewport::{
            camera::Camera,
            ray_color::{light_biased_ray_cast, light_biased_ray_color, ray_color},
            scene::Scene,
            Viewport,
        },
    };

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
    fn test_bilateral_filter() -> ImageResult<()> {
        let (scene, lights) = make_scene();
        const WIDTH: usize = 800;
        const HEIGHT: usize = 600;
        const SAMPLES: usize = 400;
        const DEPTH: usize = 9;
        const BIASED_WEIGHT: f32 = 100.;

        let cam = Camera::new(
            WIDTH as f32 / HEIGHT as f32,
            Vec3::ZERO,
            Vec3::UP,
            Vec3::FORWARD,
            90.0,
            0.0,
        );
        let lclone = lights.clone();
        // let ray_cast = Box::new(move |r: Ray, vp: Arc<Viewport>, d: usize| {
        //     light_biased_ray_cast(r, vp, d, lclone.clone())
        // });
        let biased_ray_color = Box::new(move |r: Ray, vp: Arc<Viewport>, d: usize| {
            light_biased_ray_color(r, vp, d, lights.clone(), BIASED_WEIGHT)
        });

        let vp = Viewport::new(
            cam.clone(),
            scene.clone(),
            Arc::new(ray_color),
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

        let start = Instant::now();
        let normal = vp.render_rows_async();
        bilateral_filter(&normal, Proximity::new(10, ProximityType::Square))
            .save("test_out/bilateral/bilateral_filter.png")?;
        let end = Instant::now();
        println!("Bilateral time: {:?}", end - start);
        normal.save("test_out/bilateral/ray_color_control.png")?;
        let start = Instant::now();
        vp2.render_rows_async()
            .save("test_out/bilateral/ray_color_test.png")?;
        let end = Instant::now();
        println!("ray_cast time: {:?}", end - start);
        Ok(())
    }
}
