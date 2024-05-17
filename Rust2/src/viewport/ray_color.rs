use std::sync::Arc;

use crate::vec3::{ray::Ray, vec3::Vec3};

use super::{scene::Scene, Viewport};

pub(crate) type RayColor = &'static dyn Fn(Ray, Arc<Viewport>, usize) -> Vec3;

pub(crate) fn ray_color(r: Ray, vp: Arc<Viewport>, depth: usize) -> Vec3 {
    if depth == 0 {
        return vp.bg_color;
    }
    match vp.s.get_hit(r) {
        Some((h, o)) => {
            let color = o.color(&h);

            let reflect = o.reflect(&h);

            return color.emmited
                + ray_color(reflect.reflected, vp, depth - 1).field_wise_mult(color.multiplied)
                    / reflect.pdf;
        }
        None => return vp.bg_color,
    }
}

pub(crate) fn normal_color(r: Ray, vp: Arc<Viewport>, _: usize) -> Vec3 {
    match vp.s.get_hit(r) {
        Some((h, _)) => {
            return (h.n
                + Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                })
                * 0.5
        }
        None => return Vec3::new(0.0, 0.0, 0.0),
    }
}
