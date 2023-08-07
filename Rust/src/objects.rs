pub mod objects {

    use std::{cmp::Ordering, fmt, fmt::Debug};

    use crate::vec3::{ray::Ray, vec3::Vec3};
    use self::materials::Material;

    #[derive(Clone, Copy)]
    pub struct Hit{
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
            self.t == other.t && self.normal == other.normal && self.point == other.point && self.col_mod == other.col_mod
        }
    }
    impl PartialOrd for Hit{
        fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
                return self.t.partial_cmp(&other.t);
        }
    }

    
    pub const NO_HIT:Hit = Hit{
        t: -1.0, 
        normal: Vec3{x:0.0, y:0.0, z:0.0}, 
        point: Vec3{x:0.0, y:0.0, z:0.0}, 
        col_mod:Vec3 { x: 1.0, y: 1.0, z: 1.0 }, 
        mat: Material{metallicness: 0.0, opacity: 0.0}
    };

    pub trait Object {
        fn collide(&self, r: Ray) -> bool;

        fn collision_normal(&self, r: Ray, mint:f32, maxt:f32) -> Hit;
    }
    #[allow(dead_code)]
    pub mod materials{
        use super::Hit;
        use crate::vec3::{ray::Ray, vec3::Vec3};
        use rand::random;
        #[derive(Debug, Clone, Copy)]
        pub struct Material{
            pub metallicness:f32, 
            pub opacity:f32
        }
        impl Material{
            pub fn new(metallicness: f32, opacity:f32) -> Self{
                Self { metallicness: metallicness, opacity: opacity }
            }
            pub fn new_m(metallicness: f32) -> Self{
                Self { metallicness: metallicness, opacity: 0.0 }
            }
            pub fn on_hit(&self, h :Hit, r: Ray) -> Ray{
                if random::<f32>() < self.opacity{
                    return refract(h, r);
                }
                let sc = diffuse(h, r).direction * (1.0 - self.metallicness);
                let mut reflect = metallic(h,r);
                reflect.direction = reflect.direction * self.metallicness + sc;
                return reflect;
            }
        }

        pub const METALLIC_M: Material = Material{metallicness: 1.0, opacity: 0.0};
        pub const SCATTER_M: Material = Material{metallicness: 0.0, opacity: 0.0};
        pub const FUZZY3_M: Material = Material{metallicness: 0.7, opacity: 0.0};
        pub const EMPTY_M: Material = SCATTER_M;
        pub fn empty(_hit: Hit, _:Ray) -> Ray{
            Ray{origin:Vec3 { x: 0.0, y: 0.0, z: 0.0 }, direction:Vec3 { x: 0.0, y: 0.0, z: 0.0 }}
        }
        pub fn diffuse(hit :Hit, _: Ray) -> Ray{
            // println!("diff");
            let target = hit.normal +  Vec3::random_unit_vec();
            if target.close_to_zero(){
                return Ray{origin: hit.point, direction: hit.normal};
            }
            return Ray{origin: hit.point, direction: target};
        }
        pub fn metallic(hit :Hit, r: Ray) -> Ray{
            Ray{origin: hit.point, direction: r.direction.unit().reflect(hit.normal)}
        }
        pub fn metal_fuzzy03(hit :Hit, r: Ray) -> Ray{
            Ray{origin: hit.point, direction: (r.direction.unit().reflect(hit.normal) + Vec3::random_unit_vec() * 0.3).unit()}
        }
        pub fn refract(_hit :Hit, _: Ray) -> Ray{
            Ray{origin:Vec3 { x: 0.0, y: 0.0, z: 0.0 }, direction:Vec3 { x: 0.0, y: 0.0, z: 0.0 }}
        }
    }

    #[derive(Debug)]
    pub struct Sphere {
        pub origin: Vec3,
        pub radius: f32,
        pub col_mod: Vec3,
        pub mat: Material,
        
    }
    // impl Debug for Sphere<'_>{
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //         f.debug_struct("Sphere")
    //         .field("origin", &self.origin)
    //         .field("radius", &self.radius)
    //         .field("col_mod", &self.col_mod)
    //         .finish()
    //     }
    // }

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
            return Hit{t:x, normal:(r.at(x) - self.origin).unit(), point: r.at(x), mat: self.mat, col_mod:self.col_mod};
        }
    }
    #[allow(dead_code)]
    impl Sphere{
        pub fn new(origin:Vec3, r: f32, col_mod: Option<Vec3>, mat: Option<Material>) ->Sphere{
            Sphere {
                origin: origin,
                radius: r,
                col_mod: match col_mod{
                    Some(n) => n,
                    None => Vec3::new(1.0, 1.0, 1.0) 
                },
                mat: match mat{
                    Some(n) => n,
                    None => Material{metallicness: 0.0, opacity: 0.0} 
                },
            }
        }
    }

    

    #[cfg(test)]
    mod tests{
        use super::*;
        use image::Rgb;
        use indicatif::{ProgressBar, ProgressStyle};

        use crate::write_img::img_writer::write_img_f32;
        fn ray_color(r: Ray) -> Rgb<f32> {
            let sphere = Sphere {
                origin: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                radius: -0.5,
                col_mod: Vec3::new(0.0,0.0,0.0),
                mat: Material{metallicness: 0.0, opacity: 0.0},
            };
            if sphere.collide(r) {
                return Rgb([1.0, 0.0, 0.0]);
            }
    
            let unit_direction = r.direction.unit();
            let t = 0.5 * (-unit_direction.y + 1.0);
            return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
        }
        fn ray_color_normal(r: Ray) -> Rgb<f32> {
            let sphere = Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,},0.5, None, None);
            
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

            write_img_f32(img, "sphere_test.png".to_string());
        }
        #[test]
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
}
