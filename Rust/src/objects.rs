use std::{cmp::Ordering, fmt, fmt::Debug, ops::Add};

use self::materials::Material;
use crate::vec3::{ray::Ray, vec3::Vec3};
use json::JsonValue;

pub mod aabb;
pub mod quad;
pub mod sphere;

#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub col_mod: Vec3,
    pub mat: Material,
}

impl Debug for Hit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hit")
            .field("t", &self.t)
            .field("normal", &self.normal)
            .field("point", &self.point)
            .field("col_mod", &self.col_mod)
            .finish()
    }
}

impl PartialEq for Hit {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
            && self.normal == other.normal
            && self.point == other.point
            && self.col_mod == other.col_mod
    }
}
impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.t.partial_cmp(&other.t);
    }
}

pub const NO_HIT: Hit = Hit {
    t: -1.0,
    normal: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    point: Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    },
    col_mod: Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    },
    mat: Material {
        metallicness: 0.0,
        opacity: 0.0,
        ir: 1.0,
    },
};
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    min: f32,
    max: f32,
}
fn minf(x1: f32, x2: f32) -> f32 {
    return if x1 <= x2 { x1 } else { x2 };
}
fn maxf(x1: f32, x2: f32) -> f32 {
    return if x1 >= x2 { x1 } else { x2 };
}
impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Interval {
            min: minf(self.min, rhs.min),
            max: maxf(self.max, rhs.max),
        }
    }
}
impl Interval {
    pub fn new(x1: f32, x2: f32) -> Self {
        Self {
            min: minf(x1, x2),
            max: maxf(x1, x2),
        }
    }

    /// Returns the intersection of the function `Y = aX + b` with `self` <br>
    /// Returns an `Interval` containing two points of intersection with lines `Y = self.min` and `Y = self.max`. <br> If `a == 0` returns `None` if `b` is outside `self` or `Interval{min: f32::NEG_INFINITY, max: f32::INFINITY}` if `b` is inside
    pub fn intersect(&self, a: f32, b: f32) -> Option<Interval> {
        if a == 0.0 {
            return if self.min < b && b < self.max {
                Some(Interval {
                    min: f32::NEG_INFINITY,
                    max: f32::INFINITY,
                })
            } else {
                None
            };
        }
        let inv_a = 1.0 / a;
        let x1 = (self.min - b) * inv_a;
        let x2 = (self.max - b) * inv_a;
        Some(Interval::new(x1, x2))
    }
}

pub trait Object {
    fn collide(&self, r: Ray) -> bool;

    fn collision_normal(&self, r: Ray, mint: f32, maxt: f32) -> Hit;
}
#[allow(dead_code, unused_imports)]
pub mod materials {
    use super::{Hit, JsonValue};
    use crate::vec3::{ray::Ray, vec3::Vec3};
    use rand::random;
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Material {
        pub metallicness: f32,
        pub opacity: f32,
        pub ir: f32,
    }
    impl Into<JsonValue> for Material {
        fn into(self) -> JsonValue {
            json::object! {
                metallicness: self.metallicness,
                opacity: self.opacity,
                ir: self.ir
            }
        }
    }
    impl TryFrom<JsonValue> for Material {
        type Error = crate::viewport::errors::ParseError;

        fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
            Ok(Material {
                metallicness: match value["metallicness"].as_f32() {
                    Some(x) => x,
                    None => return Err(Self::Error { source: None }),
                },
                opacity: match value["opacity"].as_f32() {
                    Some(x) => x,
                    None => return Err(Self::Error { source: None }),
                },
                ir: match value["ir"].as_f32() {
                    Some(x) => x,
                    None => return Err(Self::Error { source: None }),
                },
            })
        }
    }
    impl Material {
        pub fn new(metallicness: f32, opacity: f32, ir: f32) -> Self {
            Self {
                metallicness: metallicness,
                opacity: opacity,
                ir: ir,
            }
        }
        pub fn new_m(metallicness: f32) -> Self {
            Self {
                metallicness: metallicness,
                opacity: 0.0,
                ir: 1.0,
            }
        }
        fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
            let mut cos_theta = (-uv).dot(n);
            if cos_theta > 1.0 {
                cos_theta = 1.0
            }
            let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
            let r_out_parallel = n * -(1.0 - r_out_perp.length2()).abs().sqrt();
            return r_out_perp + r_out_parallel;
        }
        fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
            // Use Schlick's approximation for reflectance.
            let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
            r0 = r0 * r0;
            return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
        }

        pub fn on_hit(&self, h: Hit, r: Ray) -> Ray {
            if self.opacity > 0.0 {
                let n;
                let front_face = if r.direction.dot(h.normal) > 0.0 {
                    n = -h.normal;
                    false
                } else {
                    n = h.normal;
                    true
                };
                let refraction_ratio = if front_face { 1.0 / self.ir } else { self.ir };

                let unit_direction = r.direction.unit();
                let mut cos_theta = (-unit_direction).dot(n);
                if cos_theta > 1.0 {
                    cos_theta = 1.0;
                }
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let reflectance = Self::reflectance(cos_theta, refraction_ratio);
                // eprintln!("ff: {} can refract: {} ref_ratio: {}", front_face, !cannot_refract, refraction_ratio);
                let direction = if cannot_refract || reflectance > random::<f32>() {
                    // eprintln!("reflect");
                    unit_direction.reflect(n)
                } else {
                    // eprintln!("ud: {:?} hn: {:?}", unit_direction, n);
                    Self::refract(unit_direction, n, refraction_ratio)
                };

                return Ray::new_with_time(h.point, direction, r.time);
            }
            // eprintln!("reflect");
            let sc = diffuse(h, r).direction * (1.0 - self.metallicness);
            let mut reflect = metallic(h, r);
            reflect.direction = reflect.direction * self.metallicness + sc;
            return reflect;
        }
    }

    pub const METALLIC_M: Material = Material {
        metallicness: 1.0,
        opacity: 0.0,
        ir: 1.0,
    };
    pub const SCATTER_M: Material = Material {
        metallicness: 0.0,
        opacity: 0.0,
        ir: 1.0,
    };
    pub const FUZZY3_M: Material = Material {
        metallicness: 0.7,
        opacity: 0.0,
        ir: 1.0,
    };
    pub const GLASS_M: Material = Material {
        metallicness: 1.0,
        opacity: 1.0,
        ir: 1.50,
    };
    pub const GLASSR_M: Material = Material {
        metallicness: 1.0,
        opacity: 1.0,
        ir: 1.0 / GLASS_M.ir,
    };
    pub const EMPTY_M: Material = SCATTER_M;
    fn empty(_hit: Hit, r: Ray) -> Ray {
        Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            time: r.time,
        }
    }
    fn diffuse(hit: Hit, r: Ray) -> Ray {
        // println!("diff");
        let target = hit.normal + Vec3::random_unit_vec();
        if target.close_to_zero() {
            return Ray {
                origin: hit.point,
                direction: hit.normal,
                time: r.time,
            };
        }
        return Ray {
            origin: hit.point,
            direction: target,
            time: r.time,
        };
    }
    fn metallic(hit: Hit, r: Ray) -> Ray {
        Ray {
            origin: hit.point,
            direction: r.direction.unit().reflect(hit.normal),
            time: r.time,
        }
    }
    fn metal_fuzzy03(hit: Hit, r: Ray) -> Ray {
        Ray {
            origin: hit.point,
            direction: (r.direction.unit().reflect(hit.normal) + Vec3::random_unit_vec() * 0.3)
                .unit(),
            time: r.time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{sphere::Sphere, *};
    use image::Rgb;
    use indicatif::{ProgressBar, ProgressStyle};

    use crate::{texture::texture::ImageTexture, write_img::img_writer::write_img_f32};
    fn ray_color(r: Ray) -> Rgb<f32> {
        let sphere = Sphere {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.5,
            col_mod: Vec3::new(0.0, 0.0, 0.0),
            mat: Material {
                metallicness: 0.0,
                opacity: 0.0,
                ir: 1.0,
            },
            velocity: Vec3::new(0.0, 0.0, 0.0),
            texture: ImageTexture::from_color(
                Vec3 {
                    z: 0.0,
                    x: 0.0,
                    y: 0.0,
                }
                .to_rgb(),
            ),
        };
        if sphere.collide(r) {
            return Rgb([1.0, 0.0, 0.0]);
        }

        let unit_direction = r.direction.unit();
        let t = 0.5 * (-unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }
    fn ray_color_normal(r: Ray) -> Rgb<f32> {
        let sphere = Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
            None,
            None,
        );

        let hit = sphere.collision_normal(r, 0.0, 1000.0);
        if hit != NO_HIT {
            let n = hit.normal;
            return Rgb([0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0)]);
        }

        let unit_direction = r.direction.unit();
        let t = 0.5 * (-unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }
    #[test]
    pub fn sphere_test() {
        let aspect_ratio = 3.0 / 2.0;
        let width = 600_u64;
        let height = (width as f32 / aspect_ratio) as u64;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2_f32 - vertical / 2_f32 - Vec3::new(0.0, 0.0, focal_length);

        let mut img: Vec<Vec<Rgb<f32>>> = Vec::new();

        let pb = ProgressBar::new(height);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#C-"),
        );
        pb.set_message("Sphere test");

        for j in 0..height {
            pb.inc(1);
            let mut row = Vec::new();
            for i in 0..width {
                let r = Ray::new(
                    origin,
                    lower_left_corner
                        + horizontal * (i as f32 / (width - 1) as f32)
                        + vertical * (j as f32 / (height - 1) as f32),
                );
                row.push(ray_color(r));
            }
            img.push(row);
        }
        pb.finish_with_message("Writing to disk");

        write_img_f32(&img, "out/sphere_test.png".to_string());
    }
    #[test]
    pub fn sphere_test_normal() {
        let aspect_ratio = 3.0 / 2.0;
        let width = 600_u64;
        let height = (width as f32 / aspect_ratio) as u64;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length: f32 = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2_f32 - vertical / 2_f32 - Vec3::new(0.0, 0.0, focal_length);

        let mut img: Vec<Vec<Rgb<f32>>> = Vec::new();

        let pb = ProgressBar::new(height);
        pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{msg} {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#C-"),
    );
        pb.set_message("Sphere normal test");

        for j in 0..height {
            pb.inc(1);
            let mut row = Vec::new();
            for i in 0..width {
                let r = Ray::new(
                    origin,
                    lower_left_corner
                        + horizontal * (i as f32 / (width - 1) as f32)
                        + vertical * ((height - 1 - j) as f32 / (height - 1) as f32),
                );
                row.push(ray_color_normal(r));
            }
            img.push(row);
        }
        pb.finish_with_message("Writing to disk");

        write_img_f32(&img, "out/normal_test.png".to_string());
    }
}
