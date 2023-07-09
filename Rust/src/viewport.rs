pub mod viewport{

    use crate::vec3::{ray::Ray, vec3::Vec3};
    use image::Rgb;
    use indicatif::{ProgressBar, ProgressStyle};

    use crate::write_img::img_writer::write_img_f32;
    use crate::objects::objects::{NO_HIT, Sphere, Object};

    type Img = Vec<Vec<Rgb<f32>>>;

    pub struct Viewport{
        #[allow(dead_code)]
        aspect_ratio:f32,
        width :u64,
        height :u64,

        origin :Vec3,
        horizontal :Vec3,
        vertical:Vec3,
        lower_left_corner: Vec3,
    }
    
    pub struct Scene{
        spheres: Vec<Sphere>
    }

    impl Viewport{
        pub fn new(width:u64, aspect_ratio:f32)-> Self{
            
            let viewport_height = 2.0;
            let viewport_width = aspect_ratio * viewport_height;
            let focal_length: f32 = 1.0;

            let origin = Vec3::new(0.0, 0.0, 0.0);
            let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
            let vertical = Vec3::new(0.0, viewport_height, 0.0);
            let lower_left_corner =
                origin - horizontal / 2_f32 - vertical / 2_f32 - Vec3::new(0.0, 0.0, focal_length);
            
            Self{
                aspect_ratio: aspect_ratio,
                width: width,
                height: (width as f32 / aspect_ratio) as u64,

                origin:origin,
                horizontal:horizontal,
                vertical: vertical,

                lower_left_corner:lower_left_corner,
            }
        }
        pub fn new_from_res(width:u64, height:u64) -> Self{
            Self::new(width, width as f32 / height as f32)
        }

        pub fn render(&self, ray_color: &dyn Fn(Ray, &Scene)->Rgb<f32>, scene: Scene) -> Img{
            let mut img: Img = Vec::new();
            let pb = ProgressBar::new(self.height);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                    )
                    .unwrap()
                    .progress_chars("#C-"),
            );

            for j in 0..self.height {
                pb.inc(1);
                let mut row = Vec::new();
                for i in 0..self.width {
                    let r = Ray::new(
                        self.origin,
                        self.lower_left_corner
                            + self.horizontal * (i as f32 / (self.width - 1) as f32)
                            + self.vertical * ((self.height - 1 - j) as f32 / (self.height - 1) as f32),
                    );
                    row.push(ray_color(r, &scene));
                }
                img.push(row);
            }
            pb.finish_with_message("Img ready");
            return img;
        }
    }


    fn ray_color(r: Ray, scene: &Scene) -> Rgb<f32> {
        
        let mint = 0.0; let maxt = 1000.0;
        
        let hit = {
            let mut min_hit = scene.spheres[0].collision_normal(r, mint, maxt);
            for i in scene.spheres[..].into_iter().map(|sp| sp.collision_normal(r, mint, maxt)){

                if i == NO_HIT{continue;}
                if min_hit == NO_HIT{
                    min_hit = i;
                    
                }else if min_hit > i {
                    min_hit = i;
                }
            }
            min_hit
        };

        if hit != NO_HIT {
            let n = hit.normal;
            return Rgb([0.5 * (n.x + 1.0), 0.5 * (n.y + 1.0), 0.5 * (n.z + 1.0)]);
        }

        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }    


    pub fn test_viewport_object(){

        let spheres  = vec!{
            Sphere{
                origin: Vec3 {
                    x: -0.5,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            },
            Sphere{
                origin: Vec3 {
                    x: 0.50,
                    y: 0.0,
                    z: -1.0,
                },
                radius: 0.5,
            },
            Sphere{
                origin: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -2.0,
                },
                radius: 1.0,
            },
            Sphere{
                origin: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -0.0,
                },
                radius: 1.0,
            }
        };
        let scene = Scene{spheres: spheres};
        let viewport = Viewport::new_from_res(800, 600);

        let img = viewport.render(&ray_color, scene);

        write_img_f32(img, "viewport_object.png".to_string());

    }
}