use std::sync::Arc;

use crate::vec3::{ray, vec3::Vec3};

use super::{
    aabb::Interval,
    hit::Hit,
    material::Material,
    texture::{ColorResult, Texture},
    Object,
};

#[derive(Clone)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub mat: Arc<dyn Material>,
    pub texture: Arc<dyn Texture>,
}

impl Object for Sphere {
    fn get_aabb(
        &self,
    ) -> (
        super::aabb::Interval,
        super::aabb::Interval,
        super::aabb::Interval,
    ) {
        (
            Interval::new(self.origin.x - self.radius, self.origin.x + self.radius),
            Interval::new(self.origin.y - self.radius, self.origin.y + self.radius),
            Interval::new(self.origin.z - self.radius, self.origin.z + self.radius),
        )
    }

    fn get_hit(&self, r: crate::vec3::ray::Ray, mint: f32, maxt: f32) -> Option<super::hit::Hit> {
        let origin = self.origin;
        let oc = r.origin - origin;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - a * c;

        if d < 0.0 {
            return None;
        }

        let mut x = (-b - d.sqrt()) / a;
        if x < mint {
            x = (-b + d.sqrt()) / a
        }

        if x < mint || x > maxt {
            return None;
        }
        let normal = (r.at(x) - origin).unit();
        return Some(Hit {
            r,
            p: r.at(x),
            n: normal,
            t: x,
        });
    }

    fn reflect(&self, h: &Hit) -> ray::Ray {
        self.mat.on_hit(h)
    }

    fn color(&self, h: &Hit) -> ColorResult {
        let u: f32 =
            (f32::atan2(-h.n.z, h.n.x) + std::f32::consts::PI) * std::f32::consts::FRAC_1_PI * 0.5;
        let v: f32 = 1.0 - (std::f32::consts::FRAC_1_PI * f32::acos(-h.n.y));

        debug_assert!(u <= 1.0 && v >= 0.0, "U too big");
        debug_assert!(v <= 1.0 && v >= 0.0, "V too big");
        self.texture.color_at(u, v)
    }

    fn generator_pdf(&self, h: &Hit, r: &ray::Ray) -> f32 {
        self.mat.generator_pdf(h, r)
    }

    fn material_pdf(&self, h: &Hit, r: &ray::Ray) -> f32 {
        self.mat.material_pdf(h, r)
    }
}
