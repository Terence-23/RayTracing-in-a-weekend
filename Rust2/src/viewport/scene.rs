use std::sync::Arc;

use crate::{
    objects::{aabb::AABB, hit::Hit, instance::Instance, Object},
    vec3::ray::Ray,
};

#[derive(Clone)]
pub struct Scene {
    // objects: Vec<Instance>,
    aabb: AABB,
    pub(crate) mint: f32,
    pub(crate) maxt: f32,
}
impl Scene {
    pub fn get_hit(&self, r: Ray) -> Option<(Hit, Arc<dyn Object>)> {
        self.aabb.get_hit(r, self)
    }

    pub(crate) fn new(objects: Vec<Instance>, mint: f32, maxt: f32) -> Self {
        Self {
            // objects: objects.clone(),
            aabb: AABB::new(objects),
            mint,
            maxt,
        }
    }
}
