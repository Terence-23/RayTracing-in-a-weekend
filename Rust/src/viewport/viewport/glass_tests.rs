use super::*;
    use crate::write_img::img_writer::write_img_f32;
    use crate::objects::objects::materials::{EMPTY_M, SCATTER_M, METALLIC_M, FUZZY3_M, GLASS_M};

    const WIDTH: u64 = 10;
    const HEIGHT: u64 = 10;

    fn ray_color(r: Ray, scene: &Scene, depth:usize) -> Rgb<f32> {
        eprintln!("D: {}", depth);
        if depth < 1{
            
            return Rgb([0.0, 0.0, 0.0])
        }
        let mint = 0.001; let maxt = 1000.0;
        
        let hit = {
            let mut min_hit = scene.spheres[0].collision_normal(r, mint, maxt);
            for i in scene.spheres[..].into_iter().map(|sp| sp.collision_normal(r, mint, maxt)){

                if i == NO_HIT{continue;}
                if min_hit == NO_HIT || min_hit > i {
                    min_hit = i;
                }
            }
            min_hit
        };

        if hit != NO_HIT {
            eprintln!("Hit");
            if hit.mat.metallicness != 1.0 {
                return Rgb([1.0, 1.0, 0.0])
            }
            let cm = hit.col_mod;
            let front = if r.direction.dot(hit.normal) > 0.0 {
                false
            } else {
                true
            };
            let mut next = hit.mat.on_hit(hit, r);
            if next.direction.close_to_zero(){
                next.direction = if front {hit.normal} else {hit.normal * -1.0};
            }
            eprintln!("{:?}", next);
            // println!("depth: {}", depth);
            return (Vec3::from_rgb(ray_color(next, scene, depth-1)) * cm).to_rgb();
        }
        eprintln!("Sky");
        let unit_direction = r.direction.unit();
        let _t = 0.5 * (unit_direction.y + 1.0);
        return Rgb([0.0, 0.0, 1.0]); //(1.0-t)*color(1.0, 1.0, 1.0) + t*color(0.5, 0.7, 1.0);
    }

    fn glass_test_controll(){
        let samples = 100;
        let spheres  = vec!{
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(METALLIC_M)),
            Sphere::new(Vec3 {x: 0.0, y: -100.5, z: -1.0,}, 100.0, Some(Vec3::new(0.8, 0.5, 1.0)), Some(EMPTY_M)),
        };
        let scene = Scene::new(spheres);
        let viewport = Viewport::new_from_res(WIDTH, HEIGHT, samples, 10, 2.0, None, None, None, None, Some("Control for dielectric test".to_string()), None);

        let img = viewport.render_no_rand(&ray_color, &scene);

        write_img_f32(&img, "out/s_glass_test_c.png".to_string());

    }
    fn glass_test(){
        let samples = 100;
        let spheres  = vec!{
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(GLASS_M)),
            Sphere::new(Vec3 {x: 0.0, y: -100.5, z: -1.0,}, 100.0, Some(Vec3::new(1.0, 1.0, 1.0)), Some(EMPTY_M)),
        };
        let scene = Scene::new(spheres);
        let viewport = Viewport::new_from_res(WIDTH, HEIGHT, samples, 10, 2.0, None, None, None, None, Some("Dielectric test".to_string()), None);

        let img = viewport.render_no_rand(&ray_color, &scene);

        write_img_f32(&img, "out/s_glass_test.png".to_string());
        
    }
    
    use image::GenericImageView;
    use image::io::Reader;
    #[test]
    #[ignore = "used for debugging"]
    fn runner(){

        glass_test_controll();
        glass_test();
        
        let r_controll_image = match Reader::open("./s_glass_test_c.png"){
            Ok(s) => match s.decode(){
                Ok(s2) => s2,
                Err(_) => panic!("cannot read image")
            },
            Err(_) => panic!("cannot read image")
        };
        let c_controll_image = match Reader::open("./test_c.ppm"){
            Ok(s) => match s.decode(){
                Ok(s2) => s2,
                Err(_) => panic!("cannot read image")
            },
            Err(_) => panic!("cannot read image")
        };
        use std::iter::zip;
        for (rp, cp) in zip(r_controll_image.pixels(), c_controll_image.pixels()){
            eprintln!("rust: {:?}, cpp: {:?}", rp, cp);
            assert!(rp == cp, "Different controll pixels") 
        }

        assert!(r_controll_image == c_controll_image, "Different controll images");

        let r_glass_image = match Reader::open("./s_glass_test.png"){
            Ok(s) => match s.decode(){
                Ok(s2) => s2,
                Err(_) => panic!("cannot read image")
            },
            Err(_) => panic!("cannot read image")
        };
        let c_glass_image = match Reader::open("./test.ppm"){
            Ok(s) => match s.decode(){
                Ok(s2) => s2,
                Err(_) => panic!("cannot read image")
            },
            Err(_) => panic!("cannot read image")
        };

        assert!(r_glass_image == c_glass_image, "Different glass images");

    }
