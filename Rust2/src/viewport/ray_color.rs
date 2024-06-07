use std::sync::Arc;

use crate::{
    objects::Object,
    vec3::{ray::Ray, vec3::Vec3},
};

use super::{scene::Scene, Viewport};

pub(crate) type RayColor = &'static dyn Fn(Ray, Arc<Viewport>, usize) -> Vec3;

pub(crate) fn ray_color(r: Ray, vp: Arc<Viewport>, depth: usize) -> Vec3 {
    if depth == 0 {
        // dbg!(vp.bg_color);
        return vp.bg_color;
    }
    match vp.s.get_hit(r) {
        Some((h, o)) => {
            let color = o.color(&h);

            let reflect = o.reflect(&h);

            // debug_assert!(color.emmited == Vec3::zero(), "emmmited is not zero");
            let next_color = ray_color(reflect.reflected, vp, depth - 1);
            // dbg!(next_color);
            // debug_assert!(
            //     next_color.length2() >= next_color.field_wise_mult(color.multiplied).length2()
            // );

            return color.emmited + next_color.field_wise_mult(color.multiplied);
        }
        None => {
            // dbg!(vp.bg_color);
            return vp.bg_color;
        }
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

pub(crate) fn light_biased_ray_cast(
    r: Ray,
    vp: Arc<Viewport>,
    _: usize,
    lights: Arc<[Arc<dyn Object>]>,
) -> Vec3 {
    match vp.s.get_hit(r) {
        Some((h, o)) => {
            let mut count = 0;
            let mut color = Vec3::zero();
            let o_color = o.color(&h);
            for l in lights.iter() {
                let aabb = l.get_aabb();
                let middle_l = Vec3 {
                    x: aabb.0.mid_point(),
                    y: aabb.1.mid_point(),
                    z: aabb.2.mid_point(),
                };
                let to_light = (middle_l - h.p).unit();
                let r = Ray::new(h.p, to_light);
                match (vp.s.get_hit(r), l.get_hit(r, vp.s.mint, vp.s.maxt)) {
                    (Some(hr), Some(hl)) => {
                        if hr.0 == hl {
                            count += 1;
                            color += l.color(&hl).emmited;
                        }
                    }
                    _ => {}
                }
            }
            color.field_wise_mult(o_color.multiplied) / count as f32 + o_color.emmited
        }
        None => return vp.bg_color,
    }
}
