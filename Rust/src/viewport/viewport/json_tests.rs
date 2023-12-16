use super::*;
    use crate::objects::objects::materials::{SCATTER_M, METALLIC_M};

    #[test]
    fn serialize_test(){

        let scene = Scene::new(vec!
            [
                Sphere::new(Vec3 {x: -0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.6, 0.6, 0.6)), Some(SCATTER_M)),
                Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(SCATTER_M)),
                Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(METALLIC_M)),
            ]
        );
        let obj:JsonValue = scene.to_owned().into();
        // println!("obj: {:#}", obj);
        // println!("spheres: {:#}", obj["spheres"]);

        let json_s = match Scene::try_from(obj){
            Ok(s) => s,
            Err(e) => panic!("{}" ,e)
        };
        println!("Scene: {:?}", json_s);

   }
   #[test]
   fn deserialize_test(){
        let scene = Scene::new(vec!
            [
                Sphere::new(Vec3 {x: -0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(0.6, 0.6, 0.6)), Some(SCATTER_M)),
                Sphere::new(Vec3 {x: 0.5, y: 0.0, z: -1.0,}, 0.5, Some(Vec3::new(1.0, 1.0, 1.0)), Some(SCATTER_M)),
                Sphere::new(Vec3 {x: 0.0, y: 0.0, z: -2.0,}, 1.0, Some(Vec3::new(0.5, 1.0, 0.0)), Some(METALLIC_M)),
            ]
        );
        let obj:JsonValue = scene.to_owned().into();
        println!("{:#}", obj);
        println!("{:#}", obj["spheres"]);

        let json_s = match Scene::try_from(obj){
            Ok(s) => s,
            Err(e) => panic!("{}" ,e)
        };
        println!("Scene: {:?}", json_s);

        assert_eq!(scene, json_s);
   }
