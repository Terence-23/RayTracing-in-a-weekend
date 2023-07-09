pub mod objects {

    use std::cmp::Ordering;

    use crate::vec3::{ray::Ray, vec3::Vec3};
    use image::Rgb;
    use indicatif::{ProgressBar, ProgressStyle};

    use crate::write_img::img_writer::write_img_f32;

    #[derive(PartialEq, Debug)]
    pub struct Hit{
        pub t: f32,
        pub normal: Vec3,
        pub point: Vec3
    }
    impl PartialOrd for Hit{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
                return self.t.partial_cmp(&other.t);
        }
    }

    
    pub const NO_HIT:Hit = Hit{t: -1.0, normal: Vec3{x:0.0, y:0.0, z:0.0}, point: Vec3{x:0.0, y:0.0, z:0.0}};

    pub trait Object {
        fn collide(&self, r: Ray) -> bool;

        fn collision_normal(&self, r: Ray, mint:f32, maxt:f32) -> Hit;
    }

    #[derive(Debug)]
    pub struct Sphere {
        pub origin: Vec3,
        pub radius: f32,
        
    }
    

    impl Object for Sphere {
        fn collide(&self, r: Ray) -> bool {
            let oc = r.origin - self.origin;
            let a = r.direction.dot(r.direction);
            let b = 2.0 * oc.dot(r.direction);
            let c = oc.dot(oc) - self.radius * self.radius;
            return b * b - (4.0 * a * c) > 0.0;
        }
        fn collision_normal(&self, r: Ray, mint:f32, maxt:f32) -> Hit {
            let oc = r.origin - self.origin;
            let a = r.direction.dot(r.direction);
            let b = oc.dot(r.direction);
            let c = oc.dot(oc) - self.radius * self.radius;
            let d = b * b - a * c;

            if d < 0.0 {
                return NO_HIT;
            }

            let x = (if a < 0.0 {
                -b + d.sqrt()
            } else {
                -b - d.sqrt()
            }) / a;

            if x < mint || x > maxt {
                return NO_HIT;
            }
            return Hit{t:x, normal:(r.at(x) - self.origin).unit(), point: r.at(x)};
        }
    }

    fn ray_color(r: Ray) -> Rgb<f32> {
        let sphere = Sphere {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.5,
        };
        if sphere.collide(r) {
            return Rgb([1.0, 0.0, 0.0]);
        }

        let unit_direction = r.direction.unit();
        let t = 0.5 * (-unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }

    fn ray_color_normal(r: Ray) -> Rgb<f32> {
        let sphere = Sphere {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: -0.5,
        };
        let hit = sphere.collision_normal(r, 0.0, 1000.0);
        if hit != NO_HIT {
            let n = hit.normal;
            return Rgb([0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0)]);
        }

        let unit_direction = r.direction.unit();
        let t = 0.5 * (-unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }    

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
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#C-"),
        );

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

        write_img_f32(img, "sphere_test.png".to_string());
    }
    
    pub fn sphere_test_normal(){
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
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#C-"),
        );

        for j in 0..height {
            pb.inc(1);
            let mut row = Vec::new();
            for i in 0..width {
                let r = Ray::new(
                    origin,
                    lower_left_corner
                        + horizontal * (i as f32 / (width - 1) as f32)
                        + vertical * ((height -1 -j) as f32 / (height - 1) as f32),
                );
                row.push(ray_color_normal(r));
            }
            img.push(row);
        }
        pb.finish_with_message("Writing to disk");

        write_img_f32(img, "normal_test.png".to_string());
    }
}
