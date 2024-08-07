#[allow(dead_code)]
pub mod vec3 {
    use rand::random;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

    use image::Rgb;

    use crate::rotation::Rotation;

    #[derive(Debug, Clone, Copy)]
    pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    impl PartialEq for Vec3 {
        fn eq(&self, other: &Self) -> bool {
            (*self - *other).close_to_zero()
        }
    }
    impl Neg for Vec3 {
        type Output = Self;
        fn neg(self) -> Self {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }
    impl MulAssign<f32> for Vec3 {
        fn mul_assign(&mut self, rhs: f32) {
            *self = Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            };
        }
    }
    impl DivAssign<f32> for Vec3 {
        fn div_assign(&mut self, rhs: f32) {
            *self = Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            };
        }
    }
    impl AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            *self = Vec3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            };
        }
    }
    impl SubAssign for Vec3 {
        fn sub_assign(&mut self, rhs: Self) {
            *self = Vec3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            };
        }
    }

    impl Mul<f32> for Vec3 {
        type Output = Self;

        fn mul(self, rhs: f32) -> Self {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }
    impl Mul for Vec3 {
        type Output = Self;

        fn mul(self, rhs: Vec3) -> Self {
            Self {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }
    impl Div<f32> for Vec3 {
        type Output = Self;

        fn div(self, rhs: f32) -> Self {
            Self {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
    impl Add for Vec3 {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            Vec3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
    impl Sub for Vec3 {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self {
            Vec3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl Vec3 {
        pub const LEFT: Vec3 = Vec3 {
            x: 1.,
            y: 0.0,
            z: 0.0,
        };
        pub const RIGHT: Vec3 = Vec3 {
            x: -1.,
            y: 0.0,
            z: 0.0,
        };
        pub const FORWARD: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        pub const BACKWARD: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        pub const UP: Vec3 = Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        };
        pub const DOWN: Vec3 = Vec3 {
            x: 0.,
            y: -1.,
            z: 0.,
        };
        pub const ZERO: Vec3 = Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        pub const BLACK: Vec3 = Vec3::ZERO;
        pub const WHITE: Vec3 = Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        };

        pub fn rotated_uni(&self, rot: impl Rotation) -> Vec3 {
            rot.rotate(self)
        }
        pub fn rotate(&mut self, rot: Self) {
            let rotated = self.rotated(rot);
            self.x = rotated.x;
            self.y = rotated.y;
            self.z = rotated.z;
        }
        pub fn rotated(&self, rot: Self) -> Self {
            // alfa
            let asin = rot.x.sin();
            let acos = rot.x.cos();
            //beta
            let bsin = rot.y.sin();
            let bcos = rot.y.cos();
            //gamma
            let csin = rot.z.sin();
            let ccos = rot.z.cos();

            Self {
                x: self.x * bcos * ccos
                    + self.y * (asin * bsin * ccos - asin * ccos)
                    + self.z * (acos * bsin * ccos + asin * csin),
                y: self.x * bcos * csin
                    + self.y * (asin * bsin * csin + acos * ccos)
                    + self.z * (acos * bsin * csin - asin * ccos),
                z: self.x * -bsin + self.y * asin * bcos + self.z * acos * bcos,
            }
        }

        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }

        pub fn length2(&self) -> f32 {
            self.x * self.x + self.y * self.y + self.z * self.z
        }
        pub fn length(&self) -> f32 {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
        pub fn dot(&self, other: Vec3) -> f32 {
            self.x * other.x + self.y * other.y + self.z * other.z
        }
        pub fn cross(&self, other: Vec3) -> Vec3 {
            Vec3 {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            }
        }
        pub fn unit(&self) -> Vec3 {
            *self / self.length()
        }

        pub fn from_rgb(col: Rgb<f32>) -> Vec3 {
            Vec3 {
                x: col.0[0],
                y: col.0[1],
                z: col.0[2],
            }
        }
        pub fn from_rgb_ref(col: &Rgb<f32>) -> Vec3 {
            Vec3 {
                x: col.0[0],
                y: col.0[1],
                z: col.0[2],
            }
        }
        pub fn to_rgb(&self) -> Rgb<f32> {
            Rgb([self.x, self.y, self.z])
        }
        pub(crate) fn to_rgb_u8(&self) -> Rgb<u8> {
            Rgb([
                (self.x * 255.99).clamp(0.0, 255.0).round() as u8,
                (self.y * 255.99).clamp(0.0, 255.0).round() as u8,
                (self.z * 255.99).clamp(0.0, 255.0).round() as u8,
            ])
        }

        pub fn random(min: f32, max: f32) -> Vec3 {
            Vec3 {
                x: random::<f32>() * (max - min) + min,
                y: random::<f32>() * (max - min) + min,
                z: random::<f32>() * (max - min) + min,
            }
        }
        pub fn random_unit_vec() -> Vec3 {
            // println!("rand_vec");
            return loop {
                let p = Vec3::random(-1.0, 1.0);
                // println!("vec: {:?}, len: {}", p, p.x * p.x + p.y * p.y + p.z * p.z);
                if (p.x * p.x + p.y * p.y + p.z * p.z) <= 1.0 {
                    break p;
                }
            }
            .unit();
            // return p.unit();
        }
        pub fn random_in_unit_disk() -> Vec3 {
            // println!("rand_vec");
            return loop {
                let p = Vec3 {
                    x: random::<f32>() * 2.0 - 1.0,
                    y: random::<f32>() * 2.0 - 1.0,
                    z: 0.0,
                };
                // println!("vec: {:?}, len: {}", p, p.x * p.x + p.y * p.y + p.z * p.z);
                if (p.x * p.x + p.y * p.y) <= 1.0 {
                    break p;
                }
            };
            // return p.unit();
        }

        pub fn reflect(&self, n: Vec3) -> Vec3 {
            *self - n * 2.0 * self.dot(n)
        }
        pub fn close_to_zero(&self) -> bool {
            self.x.abs() < 1e-7 && self.y.abs() < 1e-7 && self.z.abs() < 1e-7
        }
        pub fn field_wise_mult(&self, v: Vec3) -> Vec3 {
            Vec3 {
                x: self.x * v.x,
                y: self.y * v.y,
                z: self.z * v.z,
            }
        }
        pub(crate) fn gamma_correct(&self, gamma: f32) -> Vec3 {
            Vec3 {
                x: self.x.powf(gamma),
                y: self.y.powf(gamma),
                z: self.z.powf(gamma),
            }
        }

        pub fn is_normal(&self) -> bool {
            !(self.x.is_nan() || self.y.is_nan() || self.z.is_nan())
        }
    }
}
#[allow(dead_code)]
pub mod ray {
    use crate::{rotation::Rotation, vec3::vec3::Vec3};
    use image::Rgb;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Ray {
        pub origin: Vec3,
        pub direction: Vec3,
        pub time: f32,
    }

    impl Ray {
        pub fn new(origin: Vec3, direction: Vec3) -> Self {
            Self {
                origin,
                direction,
                time: 0.0,
            }
        }
        pub fn new_with_time(origin: Vec3, direction: Vec3, time: f32) -> Self {
            Self {
                origin,
                direction,
                time,
            }
        }

        pub fn at(&self, t: f32) -> Vec3 {
            self.origin + self.direction * t
        }
        pub fn rotated(self, rot: impl Rotation) -> Self {
            Self {
                origin: rot.rotate(&self.origin),
                direction: rot.rotate(&self.direction),
                time: self.time,
            }
        }
    }

    fn ray_color(r: Ray) -> Rgb<f32> {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (-unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }
    #[allow(unused_imports)]
    mod tests {
        use super::*;

        use crate::write_img::img_writer::write_img_f32;
        use indicatif::{ProgressBar, ProgressStyle};
        #[test]
        pub fn viewport_test() {
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
            pb.set_message("Viewport test");

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

            write_img_f32(&img, "test_out/viewport_test.png".to_string());
        }

        #[test]
        pub fn rotation_tests() {
            const PI: f32 = std::f32::consts::PI;
            let vec = Vec3::new(1.0, 0.0, 0.0);

            let rot1 = Vec3 {
                x: PI / 6.0,
                y: 0.0,
                z: 0.0,
            };

            assert_eq!(
                vec.rotated(rot1),
                Vec3 {
                    x: rot1.y.cos() * rot1.z.cos(),
                    y: 0.0,
                    z: -0.0
                }
            );
            let rot2 = Vec3 {
                x: 0.0,
                y: 0.0,
                z: PI / 6.0,
            };
            let vec2 = Vec3::new(1.0, 0.0, 1.0);
            assert_eq!(
                vec.rotated(rot2),
                Vec3 {
                    x: rot2.z.cos(),
                    y: rot2.z.sin(),
                    z: 0.0
                }
            );
            assert_eq!(
                vec2.rotated(rot2),
                Vec3 {
                    x: rot2.z.cos(),
                    y: rot2.z.sin(),
                    z: 1.0
                }
            )
        }
    }
}
