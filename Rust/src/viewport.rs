#[allow(dead_code)]
pub mod errors {
    use std::error::Error;

    #[derive(Debug)]
    pub struct ParseError {
        pub source: Option<json::Error>,
    }
    impl std::fmt::Display for ParseError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "ParseError: {}",
                match &self.source {
                    Some(e) => e.to_string(),
                    None => "None".to_string(),
                }
            )
        }
    }
    impl Error for ParseError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
}

pub mod ray_color;

use std::iter::zip;

use crate::objects::aabb::IAABB;
use crate::objects::instance::Instance;
use crate::objects::quad::Quad;
use crate::objects::{sphere::Sphere, Object, NO_HIT};
use crate::objects::{Hit, Interval};
use crate::{
    objects::aabb::{QuadAABB, AABB},
    vec3::{ray::Ray, vec3::Vec3},
};
use image::Rgb;
use indicatif::{ProgressBar, ProgressStyle};
use json::JsonValue;
use rand::Rng;

pub type Img = Vec<Vec<Rgb<f32>>>;

#[derive(Debug, Clone)]
pub struct Viewport {
    #[allow(dead_code)]
    pub samples: usize,
    pub aspect_ratio: f32,
    pub width: u64,
    pub height: u64,

    origin: Vec3,
    upper_left_corner: Vec3,

    w: Vec3,
    u: Vec3,
    v: Vec3,

    p_delta_u: Vec3,
    p_delta_v: Vec3,

    focal_length: f32,
    lens_radius: f32,

    pub depth: usize,
    pub gamma: f32,
    pub msg: String,
    pub shutter_speed: f32,
    pub fps: f32,
    pub frame: usize,
    pub number_of_frames: usize,
    pub start_frame: usize,
}

#[derive(Debug, Clone)]
pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub aabb: AABB,
    pub quads: Vec<Quad>,
    pub qaabb: QuadAABB,
    pub instances: Vec<Instance>,
    pub iaabb: IAABB,
    pub background_color: Vec3,
}
impl Scene {
    pub fn new_sphere(spheres: Vec<Sphere>) -> Scene {
        Scene {
            spheres: spheres.clone(),
            aabb: AABB::new(spheres),
            quads: vec![],
            qaabb: QuadAABB::empty(),
            background_color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            instances: vec![],
            iaabb: IAABB::empty(),
        }
    }
    pub fn new_quad(quads: Vec<Quad>) -> Scene {
        Scene {
            spheres: vec![],
            aabb: AABB::empty(),
            quads: quads.to_owned(),
            qaabb: QuadAABB::new(quads),
            background_color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            instances: vec![],
            iaabb: IAABB::empty(),
        }
    }
    pub fn new(spheres: Vec<Sphere>, quads: Vec<Quad>, instances: Vec<Instance>) -> Self {
        Scene {
            spheres: spheres.clone(),
            aabb: AABB::new(spheres),
            quads: quads.to_owned(),
            qaabb: QuadAABB::new(quads),
            instances: instances.to_owned(),
            iaabb: IAABB::new(instances),

            background_color: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    pub fn collision_normal(&self, r: Ray, mint: f32, maxt: f32) -> Option<Hit> {
        let mut min_hit = None;
        let s_hit = self.aabb.collision_normal(r, mint, maxt);
        let q_hit = self.qaabb.collision_normal(r, mint, maxt);
        let i_hit = self.iaabb.collision_normal(r, mint, maxt);
        for i in vec![s_hit, q_hit, i_hit] {
            if i == None {
                continue;
            }
            if min_hit == None || min_hit > i {
                min_hit = i;
            }
        }
        min_hit
    }
}

impl PartialEq for Scene {
    fn eq(&self, other: &Self) -> bool {
        for (i, o) in zip(self.spheres.to_owned(), other.spheres.to_owned()) {
            if i != o {
                return false;
            }
        }
        return true;
    }
}

impl Into<JsonValue> for Scene {
    fn into(self) -> JsonValue {
        json::object! {
            spheres:self.spheres
        }
    }
}
impl TryFrom<JsonValue> for Scene {
    type Error = errors::ParseError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        let mut spheres: Vec<Sphere> = Vec::new();

        if !value["spheres"].is_array() {
            return Err(Self::Error { source: None });
        }

        for i in value["spheres"].members() {
            let sphere: Result<Sphere, <Sphere as TryFrom<JsonValue>>::Error> =
                TryInto::try_into(i.to_owned());
            match sphere {
                Ok(s) => spheres.push(s),
                Err(e) => {
                    eprintln!("Sphere error: {i}, {e}");
                    return Err(e);
                }
            }
        }

        Ok(Scene::new_sphere(spheres))
    }
}

fn gamma_correct(col: Vec3, gamma: f32) -> Vec3 {
    Vec3 {
        x: col.x.powf(gamma),
        y: col.y.powf(gamma),
        z: col.z.powf(gamma),
    }
}

pub async fn async_render(
    viewport: Box<Viewport>,
    ray_color: impl Fn(Ray, &Scene, usize) -> Rgb<f32> + std::marker::Send + std::marker::Copy + 'static,
    scene: Box<Scene>,
) -> Img {
    let mut img: Img = Vec::with_capacity(viewport.height as usize);
    let mut tasks = Vec::with_capacity(viewport.height as usize);
    let pb = ProgressBar::new(viewport.height);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#C-"),
    );
    pb.set_message(viewport.msg.to_owned());
    let inv_g = 1.0 / viewport.gamma;
    let viewport = Box::leak(viewport);
    let scene = Box::leak(scene);

    for j in 0..(viewport.height as usize) {
        tasks.push(tokio::spawn(render_row(
            viewport, ray_color, scene, inv_g, j,
        )));
    }
    for t in tasks {
        img.push(t.await.unwrap());
        pb.inc(1);
    }
    pb.finish_with_message("Img ready");

    return img;
}
pub async fn render_multi(
    viewport: Viewport,
    ray_color: impl Fn(Ray, &Scene, usize) -> Rgb<f32> + std::marker::Send + std::marker::Copy + 'static,
    scene: Scene,
) -> Vec<Img> {
    let mut viewport: Viewport = viewport.clone();
    let mut video = Vec::new();
    for i in viewport.start_frame..viewport.number_of_frames + viewport.start_frame {
        viewport.frame = i;
        viewport.msg = format!("Rendering, num: {}", i);
        video.push(
            async_render(
                Box::new(viewport.clone()),
                ray_color,
                Box::new(scene.clone()),
            )
            .await,
        );
    }
    return video;
}
async fn render_row(
    viewport: &Viewport,
    ray_color: impl Fn(Ray, &Scene, usize) -> Rgb<f32>,
    scene: &Scene,
    inv_g: f32,
    j: usize,
) -> Vec<Rgb<f32>> {
    let mut row = Vec::with_capacity(viewport.width as usize);
    let mut rng = rand::thread_rng();
    let time = viewport.frame as f32 / viewport.fps;

    for i in 0..viewport.width {
        let mut color = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        for _ in 0..viewport.samples {
            let random_point = Vec3::random_in_unit_disk();

            let r = Ray::new_with_time(
                viewport.origin
                    + (viewport.u * random_point.x + viewport.v * random_point.y)
                        * viewport.lens_radius,
                viewport.upper_left_corner
                    + viewport.p_delta_u * (i as f32 + rng.gen::<f32>())
                    + viewport.p_delta_v * (j as f32 + rng.gen::<f32>()),
                time + viewport.shutter_speed * rng.gen::<f32>(),
            );
            color += Vec3::from_rgb(ray_color(r, &scene, viewport.depth));
        }
        row.push(gamma_correct(color / viewport.samples as f32, inv_g).to_rgb());
    }

    row
}

impl Viewport {
    pub fn new(
        width: u64,
        aspect_ratio: f32,
        samples: usize,
        depth: usize,
        gamma: f32,
        vfov: Option<f32>,
        origin: Option<Vec3>,
        direction: Option<Vec3>,
        vup: Option<Vec3>,
        msg: Option<String>,
        lens_radius: Option<f32>,
    ) -> Self {
        let c_origin = match origin {
            Some(v) => v,
            None => Vec3::new(0.0, 0.0, 0.0),
        };
        let c_dir = match direction {
            Some(v) => v,
            None => Vec3::new(0.0, 0.0, -1.0),
        };
        let c_vup = match vup {
            Some(v) => v,
            None => Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };
        let c_vfov = match vfov {
            Some(x) => x,
            None => 90.0,
        };

        let w = -c_dir;
        let u = c_vup.cross(w).unit();
        let v = w.cross(u);

        let height = (width as f32 / aspect_ratio) as u64;

        let h = (c_vfov * std::f32::consts::PI / 360.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / width as f32;
        let pixel_delta_v = viewport_v / height as f32;

        let viewport_upper_left = -w - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        // let vertical = Vec3::new(0.0, viewport_height, 0.0);
        // let lower_left_corner =
        //     c_origin - horizontal / 2_f32 - vertical / 2_f32 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            samples: samples,
            aspect_ratio: aspect_ratio,
            width: width,
            height: height,

            origin: c_origin,

            w: w,
            v: v,
            u: u,

            p_delta_u: pixel_delta_u,
            p_delta_v: pixel_delta_v,

            upper_left_corner: pixel00_loc,
            lens_radius: match lens_radius {
                Some(n) => n,
                None => 0.0,
            },
            focal_length: c_dir.length(),

            depth: depth,
            gamma: gamma,
            msg: match msg {
                Some(n) => n,
                None => "".to_string(),
            },
            shutter_speed: 0.0,
            fps: 30.0,
            frame: 0,
            number_of_frames: 1,
            start_frame: 0,
        }
    }
    pub fn new_from_res(
        width: u64,
        height: u64,
        samples: usize,
        depth: usize,
        gamma: f32,
        vfov: Option<f32>,
        origin: Option<Vec3>,
        direction: Option<Vec3>,
        vup: Option<Vec3>,
        msg: Option<String>,
        lens_radius: Option<f32>,
    ) -> Self {
        Self::new(
            width,
            width as f32 / height as f32,
            samples,
            depth,
            gamma,
            vfov,
            origin,
            direction,
            vup,
            msg,
            lens_radius,
        )
    }

    pub fn render(&self, ray_color: &dyn Fn(Ray, &Scene, usize) -> Rgb<f32>, scene: &Scene) -> Img {
        let mut img: Img = Vec::with_capacity(self.height as usize);
        let pb = ProgressBar::new(self.height);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#C-"),
        );
        pb.set_message(self.msg.to_owned());
        let inv_g = 1.0 / self.gamma;

        for j in 0..self.height {
            pb.inc(1);
            let mut row = Vec::new();
            let mut rng = rand::thread_rng();

            for i in 0..self.width {
                let mut color = Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for _ in 0..self.samples {
                    let random_point = Vec3::random_in_unit_disk();

                    let r = Ray::new(
                        self.origin
                            + (self.u * random_point.x + self.v * random_point.y)
                                * self.lens_radius,
                        self.upper_left_corner
                            + self.p_delta_u * (i as f32 + rng.gen::<f32>())
                            + self.p_delta_v * (j as f32 + rng.gen::<f32>()),
                    );
                    color += Vec3::from_rgb(ray_color(r, &scene, self.depth));
                }
                row.push(gamma_correct(color / self.samples as f32, inv_g).to_rgb());
            }
            img.push(row);
        }
        pb.finish_with_message("Img ready");
        return img;
    }
    pub fn render_no_rand(
        &self,
        ray_color: &dyn Fn(Ray, &Scene, usize) -> Rgb<f32>,
        scene: &Scene,
    ) -> Img {
        let mut img: Img = Vec::new();
        let pb = ProgressBar::new(self.height);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#C-"),
        );
        pb.set_message(self.msg.to_owned());
        let inv_g = 1.0 / self.gamma;

        for j in 0..self.height {
            pb.inc(1);
            let mut row = Vec::new();
            // let mut rng = rand::thread_rng();

            for i in 0..self.width {
                // eprintln!("x: {}, y: {}\nu: {}, v: {}", i, j, u, v);
                let r = Ray::new(
                    self.origin,
                    self.upper_left_corner + self.p_delta_u * i as f32 + self.p_delta_v * j as f32,
                );
                let col = gamma_correct(Vec3::from_rgb(ray_color(r, &scene, self.depth)), inv_g);

                row.push(col.to_rgb());
            }
            img.push(row);
        }
        pb.finish_with_message("Img ready");
        return img;
    }
}

#[cfg(test)]
mod tests {
    use super::{Object, Ray, Rgb, Scene, Sphere, Vec3, Viewport, NO_HIT};
    use crate::write_img::img_writer::write_img_f32;

    fn ray_color(r: Ray, scene: &Scene, _: usize) -> Rgb<f32> {
        let mint = 0.0;
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
                if min_hit == None {
                    min_hit = i;
                } else if min_hit > i {
                    min_hit = i;
                }
            }
            min_hit
        };

        if let Some(hit) = hit {
            let n = hit.normal;
            return Rgb([0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0)]);
        }

        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }

    #[test]
    pub fn test_viewport_object() {
        let samples = 100;
        let spheres = vec![
            Sphere::new(
                Vec3 {
                    x: -0.50,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
                None,
                None,
            ),
            Sphere::new(
                Vec3 {
                    x: 0.50,
                    y: 0.0,
                    z: -1.0,
                },
                0.5,
                None,
                None,
            ),
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -2.0,
                },
                1.0,
                None,
                None,
            ),
            Sphere::new(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                1.0,
                None,
                None,
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
            Some("Viewport object test".to_string()),
            None,
        );

        let img = viewport.render(&ray_color, &scene);

        write_img_f32(&img, "out/viewport_object.png".to_string());
    }
}

#[cfg(test)]
mod camera_tests;
#[cfg(test)]
#[allow(unused_imports)]
mod glass_tests;
#[cfg(test)]
mod json_tests;
#[cfg(test)]
mod material_tests;
#[cfg(test)]
#[allow(unused_imports)]
mod texture_test;
