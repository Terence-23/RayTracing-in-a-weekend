use rand::random;

use crate::{
    objects::{maxf, minf, sphere::Sphere, Hit, Interval, Object},
    vec3::ray::Ray,
};

use super::Axis;

#[derive(Debug, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
    spheres: Vec<Sphere>,
    aabbs: Vec<AABB>,
}
#[allow(dead_code)]
impl AABB {
    pub fn new(mut spheres: Vec<Sphere>) -> Self {
        if spheres.len() == 0 {
            return Self::empty();
        }
        if spheres.len() == 1 {
            return AABB {
                spheres: spheres.to_owned(),
                x: Interval::new(
                    spheres[0].origin.x - spheres[0].radius,
                    spheres[0].origin.x + spheres[0].radius,
                ),
                y: Interval::new(
                    spheres[0].origin.y - spheres[0].radius,
                    spheres[0].origin.y + spheres[0].radius,
                ),
                z: Interval::new(
                    spheres[0].origin.z - spheres[0].radius,
                    spheres[0].origin.z + spheres[0].radius,
                ),
                aabbs: vec![],
            };
        }
        let axis = random::<Axis>();
        match axis {
            Axis::X => spheres.sort_unstable_by(|s, oth| {
                (s.origin.x + s.radius).total_cmp(&(oth.origin.x + oth.radius))
            }),
            Axis::Y => spheres.sort_unstable_by(|s, oth| {
                (s.origin.y + s.radius).total_cmp(&(oth.origin.y + oth.radius))
            }),
            Axis::Z => spheres.sort_unstable_by(|s, oth| {
                (s.origin.z + s.radius).total_cmp(&(oth.origin.z + oth.radius))
            }),
        }
        return Self::new_from_sorted(spheres);
    }
    fn new_from_sorted(spheres: Vec<Sphere>) -> Self {
        let len = spheres.len() / 2;
        let spheres1 = spheres[0..len].to_vec();
        let spheres2 = spheres[len..].to_vec();

        let aabb1 = Self::new(spheres1);
        let aabb2 = Self::new(spheres2);

        return AABB {
            x: aabb1.x + aabb2.x,
            y: aabb1.y + aabb2.y,
            z: aabb1.z + aabb2.z,
            spheres: spheres,
            aabbs: vec![aabb1, aabb2],
        };
    }
    pub fn volume(&self) -> f32 {
        (self.x.max - self.x.min) * (self.y.max - self.y.min) * (self.z.max - self.z.min)
    }
    pub fn empty() -> Self {
        Self {
            x: Interval { min: 0.0, max: 0.0 },
            y: Interval { min: 0.0, max: 0.0 },
            z: Interval { min: 0.0, max: 0.0 },
            spheres: vec![],
            aabbs: vec![],
        }
    }
}

impl Object for AABB {
    fn collide(&self, r: Ray) -> bool {
        let x_hit = match self.x.intersect(r.direction.x, r.origin.x) {
            Some(n) => n,
            None => return false,
        };
        let y_hit = match self.y.intersect(r.direction.y, r.origin.y) {
            Some(n) => n,
            None => return false,
        };
        let z_hit = match self.z.intersect(r.direction.z, r.origin.z) {
            Some(n) => n,
            None => return false,
        };
        let min = maxf(maxf(x_hit.min, y_hit.min), z_hit.min);
        let max = minf(minf(x_hit.max, y_hit.max), z_hit.max);
        min < max
    }

    fn collision_normal(&self, r: Ray, mint: f32, maxt: f32) -> Option<Hit> {
        if self.spheres.is_empty() {
            return None;
        }
        let x_hit = match self.x.intersect(r.direction.x, r.origin.x) {
            Some(n) => n,
            None => return None,
        };
        let y_hit = match self.y.intersect(r.direction.y, r.origin.y) {
            Some(n) => n,
            None => return None,
        };
        let z_hit = match self.z.intersect(r.direction.z, r.origin.z) {
            Some(n) => n,
            None => return None,
        };
        let min = maxf(maxf(x_hit.min, y_hit.min), maxf(z_hit.min, mint));
        let max = minf(minf(x_hit.max, y_hit.max), minf(z_hit.max, maxt));

        if min > max {
            return None;
        }
        let mut min_hit = None;
        if self.aabbs.len() > 0 {
            for i in self.aabbs[..]
                .into_iter()
                .map(|aabb| aabb.collision_normal(r, mint, maxt))
            {
                if i == None {
                    continue;
                }
                if min_hit == None || min_hit > i {
                    min_hit = i;
                }
            }
        } else {
            for i in self.spheres[..]
                .into_iter()
                .map(|sp| sp.collision_normal(r, mint, maxt))
            {
                if i == None {
                    continue;
                }
                if min_hit == None || min_hit > i {
                    min_hit = i;
                }
            }
        }

        return min_hit;
    }
}
