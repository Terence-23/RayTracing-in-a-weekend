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
            mat: Material{metallicness: 0.0, opacity: 0.0, ir: 1.0},
            velocity: Vec3::new(0.0,0.0,0.0),
            texture: 
                ImageTexture::from_color(
                    Vec3{z: 0.0, x: 0.0, y:0.0}.to_rgb()
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

        write_img_f32(&img, "out/sphere_test.png".to_string());
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

    write_img_f32(&img, "out/normal_test.png".to_string());
}
