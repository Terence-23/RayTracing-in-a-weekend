use std::sync::Arc;

use crate::{
    objects::{aabb::maxf, Object},
    vec3::{ray::Ray, vec3::Vec3},
};

use super::Viewport;

pub(crate) type RayColor = &'static dyn Fn(Ray, Arc<Viewport>, usize) -> Vec3;
#[allow(unused)]
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
            let next_color = ray_color(reflect, vp, depth - 1);
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
#[allow(unused)]
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
// Doesn't work with mirrors  and refraction. For those use thhe next function
#[allow(unused)]
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
                debug_assert!(
                    r.direction.is_normal(),
                    "rc, dir is nan {:?} -> {:?} -> {:?}",
                    middle_l - h.p,
                    to_light,
                    r.direction
                );
                match (vp.s.get_hit(r), l.get_hit(r, vp.s.mint, vp.s.maxt)) {
                    (Some(hr), Some(hl)) => {
                        if hr.0 == hl {
                            let pdf = o.material_pdf(&h, &r);
                            let distance2 = hl.t * hl.t * r.direction.length2();
                            count += 1;
                            color += l.color(&hl).emmited * pdf / distance2;
                        }
                    }
                    _ => {}
                }
            }
            // debug_assert_ne!(count, 0, "No lights hit");
            let ret = (if count != 0 {
                color.field_wise_mult(o_color.multiplied) / count as f32
            } else {
                Vec3::zero()
            }) + o_color.emmited;
            debug_assert!(
                !(ret.x.is_nan() || ret.y.is_nan() || ret.z.is_nan()),
                "something is nan"
            );
            return ret;
        }
        None => return vp.bg_color,
    }
}

#[allow(unused)]
pub(crate) fn light_biased_ray_color(
    r: Ray,
    vp: Arc<Viewport>,
    depth: usize,
    lights: Arc<[Arc<dyn Object>]>,
    biased_weight: f32,
) -> Vec3 {
    if depth <= 0 {
        return vp.bg_color;
    }
    match vp.s.get_hit(r) {
        Some((h, o)) => {
            let mut count = 1.;
            let r = o.reflect(&h);
            debug_assert!(r.direction.is_normal(), "reflected_dir is nan: {:?}", h);
            let mut color =
                light_biased_ray_color(r, vp.clone(), depth - 1, lights.clone(), biased_weight);
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
                            let emmited = l.color(&hl).emmited;
                            let pdf = o.material_pdf(&h, &r);
                            if pdf <= (255. * maxf(maxf(emmited.x, emmited.y), emmited.z)).recip() {
                                continue;
                            }
                            let distance2 = hl.t * hl.t * r.direction.length2();
                            count += biased_weight;
                            color += emmited * pdf / distance2 * biased_weight;
                        }
                    }
                    _ => {}
                }
            }
            // debug_assert_ne!(count, 0, "No lights hit");
            let ret = color.field_wise_mult(o_color.multiplied) / count as f32 + o_color.emmited;
            debug_assert!(
                !(ret.x.is_nan() || ret.y.is_nan() || ret.z.is_nan()),
                "something is nan"
            );
            return ret;
        }
        None => return vp.bg_color,
    }
}
