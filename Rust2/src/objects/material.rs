use crate::vec3::{ray::Ray, vec3::Vec3};

pub struct ReflectResult {
    multiplied: Vec3,
    emmited: Vec3,
    reflected: Ray,
    pdf: f32,
}
