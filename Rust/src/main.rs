
mod write_img;
use write_img::img_writer::test_write;
mod vec3;
use vec3::ray::viewport_test;
mod objects;
use objects::objects::{sphere_test, sphere_test_normal};


fn main() {
    println!("Hello, world!");
    test_write();
    viewport_test();
    sphere_test();
    sphere_test_normal();
    println!("Success");
}
