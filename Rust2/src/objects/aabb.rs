use std::{
    ops::{Add, AddAssign},
    sync::Arc,
};

use rand::random;

use crate::{
    vec3::{ray::Ray, vec3::Vec3},
    viewport::scene::Scene,
};

use super::{hit::Hit, instance::Instance, Object};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    pub(super) min: f32,
    pub(super) max: f32,
}
impl Interval {
    pub fn intervals_to_bounding_vecs(x: Interval, y: Interval, z: Interval) -> (Vec3, Vec3) {
        return (
            Vec3 {
                x: x.min,
                y: y.min,
                z: z.min,
            },
            Vec3 {
                x: x.max,
                y: y.max,
                z: z.max,
            },
        );
    }
    pub fn from_vecs(v1: Vec3, v2: Vec3) -> (Interval, Interval, Interval) {
        return (
            Interval {
                min: minf(v1.x, v2.x),
                max: maxf(v1.x, v2.x),
            },
            Interval {
                min: minf(v1.y, v2.y),
                max: maxf(v1.y, v2.y),
            },
            Interval {
                min: minf(v1.z, v2.z),
                max: maxf(v1.z, v2.z),
            },
        );
    }

    pub fn mid_point(&self) -> f32 {
        (self.min + self.max) * 0.5
    }
    pub(crate) fn new(x1: f32, x2: f32) -> Interval {
        Interval {
            min: minf(x1, x2),
            max: maxf(x1, x2),
        }
    }
    pub fn pad(&self, v: f32) -> Self {
        Self {
            min: self.min - v,
            max: self.max + v,
        }
    }

    /// Returns the intersection of the function `Y = aX + b` with `self` <br>
    /// Returns an `Interval` containing two points of intersection with lines `Y = self.min` and `Y = self.max`. <br> If `a == 0` returns `None` if `b` is outside `self` or `Interval{min: f32::NEG_INFINITY, max: f32::INFINITY}` if `b` is inside
    pub fn intersect(&self, a: f32, b: f32) -> Option<Interval> {
        if a == 0.0 {
            return if self.min < b && b < self.max {
                Some(Interval {
                    min: f32::NEG_INFINITY,
                    max: f32::INFINITY,
                })
            } else {
                None
            };
        }
        let inv_a = 1.0 / a;
        let x1 = (self.min - b) * inv_a;
        let x2 = (self.max - b) * inv_a;
        Some(Interval::new(x1, x2))
    }
}
pub(crate) fn minf(x1: f32, x2: f32) -> f32 {
    return if x1 <= x2 { x1 } else { x2 };
}
pub(crate) fn maxf(x1: f32, x2: f32) -> f32 {
    return if x1 >= x2 { x1 } else { x2 };
}
impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Interval {
            min: minf(self.min, rhs.min),
            max: maxf(self.max, rhs.max),
        }
    }
}
impl AddAssign for Interval {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Add<f32> for Interval {
    type Output = Interval;

    fn add(self, rhs: f32) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}
impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) {
            // rand 0.8
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}
#[derive(Clone)]
pub struct AABB {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
    pub(crate) instances: Vec<Instance>,
    pub(crate) aabbs: Vec<AABB>,
}
impl AABB {
    pub fn new(mut instances: Vec<Instance>) -> Self {
        if instances.len() == 0 {
            return Self::empty();
        }
        if instances.len() == 1 {
            return instances[0].get_aabb();
        }
        let axis = random::<Axis>();
        match axis {
            Axis::X => instances
                .sort_unstable_by(|s, oth| (s.get_aabb().x.max).total_cmp(&(oth.get_aabb().x.max))),
            Axis::Y => instances
                .sort_unstable_by(|s, oth| (s.get_aabb().y.max).total_cmp(&(oth.get_aabb().y.max))),
            Axis::Z => instances
                .sort_unstable_by(|s, oth| (s.get_aabb().z.max).total_cmp(&(oth.get_aabb().z.max))),
        }
        let len = instances.len() / 2;

        let aabb1 = Self::new(instances[0..len].to_vec());
        let aabb2 = Self::new(instances[len..].to_vec());

        return AABB {
            x: aabb1.x + aabb2.x,
            y: aabb1.y + aabb2.y,
            z: aabb1.z + aabb2.z,
            instances: instances,
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
            instances: vec![],
            aabbs: vec![],
        }
    }

    pub(crate) fn get_hit(
        &self,
        r: Ray,
        s: &Scene,
    ) -> Option<(Hit, Arc<dyn Object + Send + Sync>)> {
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
        let min = maxf(maxf(x_hit.min, y_hit.min), maxf(z_hit.min, s.mint));
        let max = minf(minf(x_hit.max, y_hit.max), minf(z_hit.max, s.maxt));

        if min > max {
            return None;
        }
        if self.aabbs.len() > 0 {
            let mut min_h = None;
            for h in self.aabbs.iter().map(|a| a.get_hit(r, s)) {
                if h.is_none() {
                    continue;
                }
                if min_h.is_none() {
                    min_h = h;
                    continue;
                }
                let hit = h.unwrap();
                if hit.0 < min_h.clone().unwrap().0 {
                    min_h = Some(hit);
                }
            }
            return min_h;
        }
        let mut min_h = None;
        for h in self.instances.iter().map(|i| i.get_hit(r, s)) {
            if h.is_none() {
                continue;
            }
            if min_h.is_none() {
                min_h = h;
                continue;
            }
            let hit = h.unwrap();
            if hit.0 < min_h.clone().unwrap().0 {
                min_h = Some(hit);
            }
        }
        return min_h;
    }
}
