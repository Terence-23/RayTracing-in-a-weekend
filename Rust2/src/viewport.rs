use std::sync::Arc;

use image::{ImageBuffer, Rgb};
use rayon::prelude::*;

use crate::vec3::{ray::Ray, vec3::Vec3};

use self::{camera::Camera, ray_color::RayColor, scene::Scene};

pub mod camera;
pub mod ray_color;
pub mod scene;

#[derive(Clone)]
pub(crate) struct Viewport {
    cam: Camera,

    rc: RayColor,
    width: usize,
    height: usize,
    samples: usize,
    recursion_depth: usize,
    gamma: f32,
    bg_color: Vec3,
    s: Scene,
}

impl Viewport {
    pub fn new(
        cam: Camera,
        s: Scene,
        rc: RayColor,
        width: usize,
        height: usize,
        samples: usize,
        recursion_depth: usize,
        bg_color: Vec3,
        gamma: f32,
    ) -> Self {
        Self {
            cam,

            rc: rc,
            width,
            height,
            samples,
            s,
            recursion_depth,
            gamma,
            bg_color,
        }
    }
    fn make_image(iv: Vec<Vec<Vec3>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(iv[0].len() as u32, iv.len() as u32);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            *pix = iv[y as usize][x as usize].to_rgb_u8();
        }
        img
    }
    fn ray_depth(r: &Ray, scene: &Scene) -> f32 {
        let ray = Ray::new(r.origin, r.direction.unit());

        return if let Some(hit) = scene.get_hit(ray) {
            hit.0.t
        } else {
            scene.maxt * 1.6
        };
    }

    pub fn depth_map(&self, scene: &Scene) -> Vec<Vec<f32>> {
        let mut img = vec![vec![]];
        for j in 0..self.height {
            let mut tmp = vec![];
            for i in 0..self.width {
                let dir = self.cam.left_top
                    + self.cam.delta_x * (i as f32 / self.width as f32)
                    + self.cam.delta_y * (j as f32 / self.height as f32);
                tmp.push(Self::ray_depth(&Ray::new(self.cam.origin, dir), scene));
            }
            img.push(tmp);
        }
        return img;
    }

    fn render_row(self: Arc<Self>, y: usize) -> Vec<Vec3> {
        let mut row = Vec::with_capacity(self.width);
        let inv_gamma = 1.0 / self.gamma;
        let s_sqrt = (self.samples as f32).sqrt().floor() as usize;
        for j in 0..self.width {
            let mut pix = Vec3::ZERO;
            for k in 0..s_sqrt {
                for l in 0..s_sqrt {
                    let dir = self.cam.left_top
                        + self.cam.delta_x
                            * ((j as f32 + (k as f32 + 0.5) / s_sqrt as f32) / self.width as f32)
                        + self.cam.delta_y
                            * ((y as f32 + (l as f32 + 0.5) / s_sqrt as f32) / self.height as f32);
                    let r = Ray::new(
                        self.cam.origin + Vec3::random_in_unit_disk() * self.cam.lens_radius,
                        dir,
                    );
                    pix += (self.rc)(r, self.to_owned(), self.recursion_depth);
                }
            }
            // average all samples
            pix /= (s_sqrt * s_sqrt) as f32;
            // gamma correct
            let pix = pix.gamma_correct(inv_gamma);
            row.push(pix);
        }
        row
    }

    pub fn render_rows_async(self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let arc = Arc::new(self);

        let image_vec: Vec<_> = (0..arc.height)
            .into_par_iter()
            .map(|y| Self::render_row(arc.clone(), y))
            .collect();

        // self.make_image(image_vec)
        Self::make_image(image_vec)
    }

    pub fn render(self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image_vec = Vec::with_capacity(self.height);
        let s_sqrt = (self.samples as f32).sqrt().floor() as usize;
        let arc = Arc::new(self.to_owned());
        let inv_gamma = 1.0 / self.gamma;

        for i in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for j in 0..self.width {
                let mut pix = Vec3::ZERO;
                for k in 0..s_sqrt {
                    for l in 0..s_sqrt {
                        let dir = self.cam.left_top
                            + self.cam.delta_x
                                * ((j as f32 + (k as f32 + 0.5) / s_sqrt as f32)
                                    / self.width as f32)
                            + self.cam.delta_y
                                * ((i as f32 + (l as f32 + 0.5) / s_sqrt as f32)
                                    / self.height as f32);
                        let r = Ray::new(
                            self.cam.origin + Vec3::random_in_unit_disk() * self.cam.lens_radius,
                            dir,
                        );
                        pix += (self.rc)(r, arc.to_owned(), self.recursion_depth);
                    }
                }
                // average all samples
                pix /= (s_sqrt * s_sqrt) as f32;
                // gamma correct
                let pix = pix.gamma_correct(inv_gamma);
                row.push(pix);
            }
            image_vec.push(row);
        }

        Self::make_image(image_vec)
    }
}
