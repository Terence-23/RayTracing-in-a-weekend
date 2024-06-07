use std::{cell::Ref, f32::consts::PI, sync::Arc};

use crate::vec3::{ray::Ray, vec3::Vec3};

use super::hit::Hit;
use lazy_static::lazy_static;
use rand::random;

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

pub struct MirrorGlass {
    opacity: f32,
    ir: f32,
}
impl MirrorGlass {
    fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let mut cos_theta = (-uv).dot(n);
        if cos_theta > 1.0 {
            cos_theta = 1.0
        }
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(1.0 - r_out_perp.length2()).abs().sqrt();
        return r_out_perp + r_out_parallel;
    }
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}

impl Material for MirrorGlass {
    fn on_hit(&self, h: &Hit) -> ReflectResult {
        let n;
        let front_face = if h.r.direction.dot(h.n) > 0.0 {
            n = -h.n;
            false
        } else {
            n = h.n;
            true
        };
        let refraction_ratio = if front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = h.r.direction.unit();
        let mut cos_theta = (-unit_direction).dot(n);
        if cos_theta > 1.0 {
            cos_theta = 1.0;
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = Self::reflectance(cos_theta, refraction_ratio);
        // eprintln!("ff: {} can refract: {} ref_ratio: {}", front_face, !cannot_refract, refraction_ratio);
        let direction = if cannot_refract || reflectance > random::<f32>() {
            // eprintln!("reflect");
            unit_direction.reflect(n)
        } else {
            // eprintln!("ud: {:?} hn: {:?}", unit_direction, n);
            Self::refract(unit_direction, n, refraction_ratio)
        };

        return ReflectResult {
            reflected: Ray::new_with_time(h.p, direction, h.r.time),
            pdf: 1.0,
        };
    }
}

pub struct MixedMaterial {}
