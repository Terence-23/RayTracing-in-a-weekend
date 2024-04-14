use std::sync::Arc;

use crate::{
    objects::{instance::Instance, sphere::Sphere},
    vec3::vec3::Vec3,
    viewport::{camera::Camera, ray_color, scene::Scene, Viewport},
};

#[test]
fn normal_sphere_test() -> Result<(), image::ImageError> {
    let i = Instance::new(Arc::new([Arc::new(Sphere {
        origin: Vec3::zero(),
        radius: 0.5,
    })]));
    let s = Scene::new(vec![i.clone()], 0.001, 1000.0);
    let cam = Camera::new(
        1.5,
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        70.0,
        0.0,
    );

    let vp = Viewport::new(cam, s, &ray_color::normal_color, 300, 200, 25, 10, 1.0);

    vp.render().save("test_out/normal_sphere_test.png")?;
    let aabb = i.get_aabb();
    print!(
        "Sphere aabb: x: {:?}, y: {:?}, z:{:?}",
        aabb.x, aabb.y, aabb.z
    );
    Ok(())
}
