#[allow(dead_code)]
pub mod vec3 {
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg};
    use rand::random;

    use image::Rgb;
    #[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl Neg for Vec3{
        type Output = Self;
        fn neg(self) -> Self{
            Vec3 { x: -self.x, y: -self.y, z: -self.z }
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
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self { x, y, z }
        }
        pub fn to_rgb(&self) ->Rgb<f32>{
            Rgb([self.x, self.y, self.z])
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
            Vec3{x:col.0[0], y:col.0[1], z:col.0[2]}
        }
        pub fn random(min: f32, max:f32) ->Vec3{
            Vec3 { x: random::<f32>() * (max - min) + min, y: random::<f32>() * (max - min) + min, z: random::<f32>() * (max - min) + min }
        }
        pub fn random_unit_vec() ->Vec3{
            // println!("rand_vec");
            return loop{
                let p = Vec3::random(-1.0,1.0);
                // println!("vec: {:?}, len: {}", p, p.x * p.x + p.y * p.y + p.z * p.z);
                if (p.x * p.x + p.y * p.y + p.z * p.z) <= 1.0 {
                    break p;
                }
            }.unit();
            // return p.unit();
        }
        pub fn reflect(&self, n: Vec3) -> Vec3{
            *self - n * 2.0 * self.dot(n)
        }
        pub fn close_to_zero(&self) -> bool{
            self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
        }

    }
}
#[allow(dead_code)]
pub mod ray {
    use crate::vec3::vec3::Vec3;
    use image::Rgb;

    #[derive(Debug, Clone, Copy)]
    pub struct Ray {
        pub origin: Vec3,
        pub direction: Vec3,
    }

    impl Ray {
        pub fn new(origin: Vec3, direction: Vec3) -> Self {
            Self { origin, direction }
        }

        pub fn at(&self, t: f32) -> Vec3 {
            self.origin + self.direction * t
        }
    }

    fn ray_color(r: Ray) -> Rgb<f32> {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (-unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }
    #[allow(unused_imports)]
    mod tests{
        use super::*;

        use indicatif::{ProgressBar, ProgressStyle};
        use crate::write_img::img_writer::write_img_f32;
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

            write_img_f32(img, "viewport_test.png".to_string());
        }
    }

}
