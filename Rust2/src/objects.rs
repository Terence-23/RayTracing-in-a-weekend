use crate::vec3::ray::Ray;

use self::{aabb::Interval, hit::Hit, material::ReflectResult};

pub mod aabb;
pub mod hit;
pub mod instance;
pub mod material;
pub mod sphere;

pub trait CloneObject: Clone + Object {}
pub trait Object {
    fn get_aabb(&self) -> (Interval, Interval, Interval);
    fn get_hit(&self, r: Ray, mint: f32, maxt: f32) -> Option<Hit>;
    fn reflect(&self, h: Hit) -> ReflectResult;
}
