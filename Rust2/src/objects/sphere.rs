use crate::vec3::vec3::Vec3;

use super::{aabb::Interval, hit::Hit, Object};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
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

        // let u: f32 = (f32::atan2(-normal.z, normal.x) + std::f32::consts::PI)
        //     * std::f32::consts::FRAC_1_PI
        //     * 0.5;
        // let v: f32 = 1.0 - (std::f32::consts::FRAC_1_PI * f32::acos(-normal.y));
    }

    fn reflect(&self, h: super::hit::Hit) -> super::material::ReflectResult {
        todo!()
    }
}
