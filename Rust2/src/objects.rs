use crate::vec3::ray::Ray;

use self::{aabb::Interval, hit::Hit};

pub mod aabb;
pub mod hit;
pub mod instance;
pub mod material;
pub mod quad;
pub mod sphere;
pub mod texture;
pub mod triangle;

pub trait CloneObject: Clone + Object {}

pub trait Object {
    fn get_aabb(&self) -> (Interval, Interval, Interval);
    fn get_hit(&self, r: Ray, mint: f32, maxt: f32) -> Option<Hit>;
    fn reflect(&self, h: &Hit) -> Ray;
    fn generator_pdf(&self, h: &Hit, r: &Ray) -> f32;
    fn material_pdf(&self, h: &Hit, r: &Ray) -> f32;
    fn color(&self, h: &Hit) -> texture::ColorResult;
}
