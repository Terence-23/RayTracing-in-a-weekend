use std::{f32::consts::PI, sync::Arc};

use crate::vec3::{ray::Ray, vec3::Vec3};

use super::hit::Hit;
use lazy_static::lazy_static;

pub struct ReflectResult {
    pub(crate) reflected: Ray,
    pub(crate) pdf: f32,
}

pub trait Material {
    fn on_hit(&self, h: &Hit) -> ReflectResult;
}

const FRAC_1_2PI: f32 = 1.0 / 2.0 / PI;
lazy_static! {
    pub static ref LAMBERTIAN: Arc<Lambertian> = Arc::new(Lambertian {});
    pub static ref MIRROR: Arc<Mirror> = Arc::new(Mirror {});
}
pub struct Lambertian {}
impl Material for Lambertian {
    fn on_hit(&self, h: &Hit) -> ReflectResult {
        let dir = (h.n + Vec3::random_unit_vec()).unit();
        ReflectResult {
            reflected: Ray {
                origin: h.p,
                direction: dir,
                time: h.r.time,
            },
            pdf: dir.dot(h.n) * FRAC_1_2PI,
        }
    }
}

pub fn lambertian(h: &Hit) -> ReflectResult {
    let dir = (h.n + Vec3::random_unit_vec()).unit();
    ReflectResult {
        reflected: Ray {
            origin: h.p,
            direction: dir,
            time: h.r.time,
        },
        pdf: dir.dot(h.n) * FRAC_1_2PI,
    }
}

pub struct Mirror {}
impl Material for Mirror {
    fn on_hit(&self, h: &Hit) -> ReflectResult {
        ReflectResult {
            reflected: Ray {
                origin: h.p,
                direction: h.r.direction.reflect(h.n),
                time: h.r.time,
            },
            pdf: 1.0,
        }
    }
}

pub fn mirror(h: &Hit) -> ReflectResult {
    ReflectResult {
        reflected: Ray {
            origin: h.p,
            direction: h.r.direction.reflect(h.n),
            time: h.r.time,
        },
        pdf: 1.0,
    }
}

pub struct ComboReflect(f32);

impl Material for ComboReflect {
    fn on_hit(&self, h: &Hit) -> ReflectResult {
        let s_dir = (h.n + Vec3::random_unit_vec()).unit();
        let r_dir = h.r.direction.reflect(h.n).unit();

        ReflectResult {
            reflected: Ray {
                origin: h.p,
                direction: s_dir * (1.0 - self.0) + r_dir * self.0,
                time: h.r.time,
            },
            pdf: s_dir.dot(h.n) * FRAC_1_2PI,
        }
    }
}
