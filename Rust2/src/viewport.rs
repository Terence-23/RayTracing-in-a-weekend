use std::sync::Arc;

use image::{ImageBuffer, Rgb};

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
    fn make_image(&self, iv: Vec<Vec<Vec3>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> =
            ImageBuffer::new(iv[0].len() as u32, iv.len() as u32);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            *pix = iv[y as usize][x as usize].to_rgb_u8();
        }
        img
    }
    pub fn render(self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut image_vec = Vec::with_capacity(self.height);
        let s_sqrt = (self.samples as f32).sqrt().floor() as usize;
        let arc = Arc::new(self.to_owned());
        let inv_gamma = 1.0 / self.gamma;

        for i in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for j in 0..self.width {
                let mut pix = Vec3::zero();
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

        self.make_image(image_vec)
    }
}
