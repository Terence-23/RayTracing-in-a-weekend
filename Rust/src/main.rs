

#[allow(unused_imports)]
mod write_img;
use write_img::img_writer::test_write;
mod vec3;
use vec3::ray::viewport_test;
mod objects;
use objects::objects::{sphere_test, sphere_test_normal};
mod viewport;
use viewport::viewport::test_viewport_object;


fn main() {
    println!("Hello, world!");
    test_write();
    viewport_test();
    sphere_test();
    sphere_test_normal();
    test_viewport_object();
    println!("Success");
}
