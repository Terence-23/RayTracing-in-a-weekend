use super::*;
    use crate::write_img::img_writer::write_img_f32;
    use crate::objects::objects::materials::{EMPTY_M, SCATTER_M, METALLIC_M, FUZZY3_M, GLASS_M, GLASSR_M};
    #[test]
    pub fn diffuse_test(){
        let samples = 100;
        let spheres  = vec!{
            Sphere::new(Vec3 {x: -0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.6, 0.6, 0.6)), Some(SCATTER_M)),
            Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(SCATTER_M)),
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(SCATTER_M)),
        };
        let scene = Scene::new(spheres);
        let viewport = Viewport::new_from_res(800, 600, samples, 10, 2.0, None, None, None, None, Some("Diffuse test".to_string()), None);

        let img = viewport.render(&ray_color_d, scene);

        write_img_f32(&img, "out/diffuse_test.png".to_string());

    }
    #[test]
    pub fn metal_test(){
        let samples = 100;
        let spheres  = vec!{
            Sphere::new(Vec3 {x: -1.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.8, 0.8, 0.8)), Some(METALLIC_M)),
            Sphere::new(Vec3 {x: 1.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.8, 0.6, 0.2)), Some(METALLIC_M)),
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.7, 0.30, 0.30)), Some(SCATTER_M)),
            Sphere::new(Vec3 {x: 0.0, y: -100.5, z: -1.0,}, 100.0, Some(Vec3::new(0.8, 0.8, 0.0)), Some(SCATTER_M)),
        };
        let scene = Scene::new(spheres);
        let viewport = Viewport::new_from_res(400, 225, samples, 10, 2.0, None, None, None, None, Some("Metallic test".to_string()), None);

        let img = viewport.render(&ray_color_d, scene);

        write_img_f32(&img, "out/metal_test.png".to_string());

    }
    #[test]
    pub fn glass_test_controll(){
        let samples = 100;
        let spheres  = vec!{
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(METALLIC_M)),
            // Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.5, 0.9, 0.9)), Some(METALLIC_M)),
            // Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(SCATTER_M)),
            Sphere::new(Vec3 {x: 0.0, y: -100.5, z: -1.0,}, 100.0, Some(Vec3::new(0.8, 0.5, 1.0)), Some(EMPTY_M)),
        };
        let scene = Scene::new(spheres);
        let viewport = Viewport::new_from_res(300, 200, samples, 10, 2.0, None, None, None, None, Some("Control for dielectric test".to_string()), None);

        let img = viewport.render(&ray_color_d, scene);

        write_img_f32(&img, "out/glass_test_c.png".to_string());

    }
    #[test]
    pub fn glass_test(){
        let samples = 100;
        let spheres  = vec!{
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(GLASS_M)),
            Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -1.0,}, 0.35, Some(Vec3::new(1.0, 1.0, 1.0)), Some(GLASSR_M)),
            // Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.5, 0.9, 0.9)), Some(METALLIC_M)),
            // Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(SCATTER_M)),
            Sphere::new(Vec3 {x: 0.0, y: -100.5, z: -1.0,}, 100.0, Some(Vec3::new(0.8, 0.5, 1.0)), Some(EMPTY_M)),
        };
        let scene = Scene::new(spheres);
        let viewport = Viewport::new_from_res(300, 200, samples, 10, 2.0, None, None, None, None, Some("Dielectric test".to_string()), None);

        let img = viewport.render(&ray_color_d, scene);

        write_img_f32(&img, "out/glass_test.png".to_string());

    }
