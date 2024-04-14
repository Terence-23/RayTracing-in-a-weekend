use std::sync::Arc;

use crate::vec3::{ray::Ray, vec3::Vec3};

use super::{scene::Scene, Viewport};

pub(crate) type RayColor = &'static dyn Fn(Ray, Arc<Viewport>, usize) -> Vec3;

pub(crate) fn ray_color(r: Ray, vp: Arc<Viewport>, depth: usize) -> Vec3 {
    match vp.s.get_hit(r) {
        Some(h) => Vec3::new(0.0, 0.0, 0.0),
        None => Vec3::new(0.0, 0.0, 0.0),
    }
}

pub(crate) fn normal_color(r: Ray, vp: Arc<Viewport>, _: usize) -> Vec3 {
    match vp.s.get_hit(r) {
        Some((h, _)) => {
            return ((h.n
                + Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                })
                * 0.5)
        }
        None => return Vec3::new(0.0, 0.0, 0.0),
    }
}
