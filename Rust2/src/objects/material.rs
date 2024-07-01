use std::{f32::consts::PI, sync::Arc};

use crate::{
    onb::ONB,
    vec3::{ray::Ray, vec3::Vec3},
};

use super::hit::Hit;
use lazy_static::lazy_static;
use rand::random;

pub trait Material {
    fn on_hit(&self, h: &Hit) -> Ray;
    // probability of generating given reflection
    fn generator_pdf(&self, h: &Hit, r: &Ray) -> f32;
    // material probability of given reflection
    fn material_pdf(&self, h: &Hit, r: &Ray) -> f32;
}

const FRAC_1_2PI: f32 = 1.0 / 2.0 / PI;
lazy_static! {
    pub static ref LAMBERTIAN: Arc<Lambertian> = Arc::new(Lambertian {});
    pub static ref MIRROR: Arc<Mirror> = Arc::new(Mirror {});
}
pub struct Lambertian {}
impl Material for Lambertian {
    fn on_hit(&self, h: &Hit) -> Ray {
        let dir = (h.n + Vec3::random_unit_vec()).unit();

        Ray {
            origin: h.p,
            direction: dir,
            time: h.r.time,
        }
    }

    fn generator_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if r.origin != h.p {
            return 0.0;
        }
        let cos = r.direction.unit().dot(h.n.unit());
        return cos.clamp(0.0, 1.0) * core::f32::consts::FRAC_1_PI;
    }

    fn material_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if r.origin != h.p {
            // dbg!("bad origin");
            return 0.0;
        }
        let cos = r.direction.unit().dot(h.n.unit());

        // if cos <= 0.0 {
        //     dbg!(cos);
        // }
        return if h.r.direction.dot(h.n) >= 0.0 {
            -cos
        } else {
            cos
        }
        .clamp(0.0, 1.0)
            * core::f32::consts::FRAC_1_PI;
    }
}

pub fn lambertian(h: &Hit) -> Ray {
    let dir = (h.n + Vec3::random_unit_vec()).unit();

    Ray {
        origin: h.p,
        direction: dir,
        time: h.r.time,
    }
}

pub struct Mirror {}
impl Material for Mirror {
    fn on_hit(&self, h: &Hit) -> Ray {
        Ray {
            origin: h.p,
            direction: h.r.direction.reflect(h.n),
            time: h.r.time,
        }
    }

    fn generator_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if *r == self.on_hit(h) {
            1.0
        } else {
            0.0
        }
    }

    fn material_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if *r == self.on_hit(h) {
            1.0
        } else {
            0.0
        }
    }
}

pub fn mirror(h: &Hit) -> Ray {
    Ray {
        origin: h.p,
        direction: h.r.direction.reflect(h.n),
        time: h.r.time,
    }
}

pub struct MirrorGlass {
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
    fn on_hit(&self, h: &Hit) -> Ray {
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

        return Ray::new_with_time(h.p, direction, h.r.time);
    }

    fn generator_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if r.origin != h.p {
            return 0.0;
        }
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
        if cannot_refract && unit_direction.reflect(n) == r.direction {
            return 1.0;
        }
        let reflectance = Self::reflectance(cos_theta, refraction_ratio);
        if unit_direction.reflect(n) == r.direction {
            reflectance
        } else if Self::refract(unit_direction, n, refraction_ratio) == r.direction {
            1.0 - reflectance
        } else {
            0.0
        }
    }

    fn material_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if r.origin != h.p {
            return 0.0;
        }
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
        if cannot_refract && unit_direction.reflect(n) == r.direction {
            return 1.0;
        }
        let reflectance = Self::reflectance(cos_theta, refraction_ratio);
        if unit_direction.reflect(n) == r.direction {
            reflectance
        } else if Self::refract(unit_direction, n, refraction_ratio) == r.direction {
            1.0 - reflectance
        } else {
            0.0
        }
    }
}

//pdf(x) = (cos(x))^(exp) * (exp+1)/2pi
//cdf(x) = 1 - (cos(x))^(exp+1)
//gen(x) = arccos((1-x)^(1/(n+1)))
pub struct MixedMaterial {
    pub exp: f32,
    gen_exp: f32,
}
impl MixedMaterial {
    pub fn new(exp: f32) -> MixedMaterial {
        MixedMaterial {
            exp: exp,
            gen_exp: 1.0 / (exp + 1.0),
        }
    }
    pub fn gen_random_dir(&self) -> Vec3 {
        let phi = random::<f32>() * 2.0 * PI;
        let cos_theta = (1.0 - random::<f32>()).powf(self.gen_exp);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        Vec3 {
            x: phi.cos() * sin_theta,
            y: phi.sin() * sin_theta,
            z: cos_theta,
        }
    }
}
impl Material for MixedMaterial {
    fn on_hit(&self, h: &Hit) -> Ray {
        let uvw = ONB::new_from_w(h.n);
        Ray::new(h.p, uvw.from_local(self.gen_random_dir()))
    }

    fn generator_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if r.origin != h.p {
            return 0.0;
        }
        let cos = if h.r.direction.dot(h.n) < 0.0 {
            r.direction.unit().dot(h.n.unit())
        } else {
            -r.direction.unit().dot(h.n.unit())
        };

        return cos.powf(self.exp) * (self.exp + 1.0) * FRAC_1_2PI;
    }

    fn material_pdf(&self, h: &Hit, r: &Ray) -> f32 {
        if r.origin != h.p {
            // eprintln!("Bad origin");
            return 0.0;
        }
        let cos = if h.r.direction.dot(h.n) < 0.0 {
            r.direction.unit().dot(h.n.unit())
        } else {
            -r.direction.unit().dot(h.n.unit())
        };
        if cos < 0. {
            // eprintln!("negative_cos");
            return 0.;
        }

        return cos.powf(self.exp) * (self.exp + 1.0) * FRAC_1_2PI;
    }
}
