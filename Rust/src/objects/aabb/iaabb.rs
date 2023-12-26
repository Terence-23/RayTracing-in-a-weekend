use rand::random;

use crate::{
    objects::{instance::Instance, maxf, minf, Hit, Interval, Object, NO_HIT},
    vec3::ray::Ray,
};

use super::Axis;
#[derive(Debug, Clone)]
pub struct IAABB {
    x: Interval,
    y: Interval,
    z: Interval,
    instances: Vec<Instance>,
    aabbs: Vec<IAABB>,
}

impl From<Instance> for IAABB {
    fn from(val: Instance) -> Self {
        let x = val.qaabb.x + val.saabb.x + val.gett().x;
        let y = val.qaabb.y + val.saabb.y + val.gett().y;
        let z = val.qaabb.z + val.saabb.z + val.gett().z;
        Self {
            x: x.pad((x.max - x.min) / 2.0),
            y: y.pad((y.max - y.min) / 2.0),
            z: z.pad((x.max - z.min) / 2.0),
            instances: vec![val.to_owned()],
            aabbs: vec![],
        }
    }
}
impl From<&Instance> for IAABB {
    fn from(val: &Instance) -> Self {
        let x = val.qaabb.x + val.saabb.x + val.gett().x;
        let y = val.qaabb.y + val.saabb.y + val.gett().y;
        let z = val.qaabb.z + val.saabb.z + val.gett().z;
        Self {
            x: x.pad((x.max - x.min) / 2.0),
            y: y.pad((y.max - y.min) / 2.0),
            z: z.pad((x.max - z.min) / 2.0),
            instances: vec![val.to_owned()],
            aabbs: vec![],
        }
    }
}

#[allow(dead_code)]
impl IAABB {
    pub fn new(mut quads: Vec<Instance>) -> Self {
        if quads.len() == 0 {
            return Self::empty();
        }
        if quads.len() == 1 {
            return (&quads[0]).into();
        }
        let axis = random::<Axis>();
        match axis {
            Axis::X => quads.sort_unstable_by(|s, oth| {
                (IAABB::from(s).x.min).total_cmp(&(IAABB::from(oth).x.min))
            }),
            Axis::Y => quads.sort_unstable_by(|s, oth| {
                (IAABB::from(s).y.min).total_cmp(&(IAABB::from(oth).y.min))
            }),
            Axis::Z => quads.sort_unstable_by(|s, oth| {
                (IAABB::from(s).z.min).total_cmp(&(IAABB::from(oth).z.min))
            }),
        }
        return Self::new_from_sorted(quads);
    }
    fn new_from_sorted(instances: Vec<Instance>) -> Self {
        let len = instances.len() / 2;
        let instances1 = instances[0..len].to_vec();
        let instances2 = instances[len..].to_vec();

        let aabb1 = Self::new(instances1);
        let aabb2 = Self::new(instances2);

        return IAABB {
            x: aabb1.x + aabb2.x,
            y: aabb1.y + aabb2.y,
            z: aabb1.z + aabb2.z,
            instances,
            aabbs: vec![aabb1, aabb2],
        };
    }
    pub fn volume(&self) -> f32 {
        (self.x.max - self.x.min) * (self.y.max - self.y.min) * (self.z.max - self.z.min)
    }
    pub fn empty() -> Self {
        IAABB {
            x: Interval { min: 0.0, max: 0.0 },
            y: Interval { min: 0.0, max: 0.0 },
            z: Interval { min: 0.0, max: 0.0 },
            instances: vec![],
            aabbs: vec![],
        }
    }
}

impl Object for IAABB {
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

    fn collision_normal(&self, r: Ray, mint: f32, maxt: f32) -> Hit {
        if self.instances.is_empty() && self.aabbs.is_empty() {
            eprintln!("empty");
            return NO_HIT;
        }

        let x_hit = match self.x.intersect(r.direction.x, r.origin.x) {
            Some(n) => n,
            None => return NO_HIT,
        };
        let y_hit = match self.y.intersect(r.direction.y, r.origin.y) {
            Some(n) => n,
            None => return NO_HIT,
        };
        let z_hit = match self.z.intersect(r.direction.z, r.origin.z) {
            Some(n) => n,
            None => return NO_HIT,
        };
        let min = maxf(maxf(x_hit.min, y_hit.min), maxf(z_hit.min, mint));
        let max = minf(minf(x_hit.max, y_hit.max), minf(z_hit.max, maxt));

        if min > max {
            // eprintln!("min > max");
            // dbg!(min);
            // dbg!(max);
            // dbg!(x_hit, y_hit, z_hit);

            return NO_HIT;
        }
        let mut min_hit = NO_HIT;
        if self.aabbs.len() > 0 {
            for i in self.aabbs[..]
                .into_iter()
                .map(|aabb| aabb.collision_normal(r, mint, maxt))
            {
                if i == NO_HIT {
                    continue;
                }
                if min_hit == NO_HIT || min_hit > i {
                    min_hit = i;
                }
            }
        } else {
            // eprintln!("quad intersect");
            for i in self.instances[..]
                .into_iter()
                .map(|sp| sp.collision_normal(r, mint, maxt))
            {
                if i == NO_HIT {
                    continue;
                }
                if min_hit == NO_HIT || min_hit > i {
                    min_hit = i;
                }
            }

            // if min_hit == NO_HIT {
            //     eprintln!("NO_HIT");
            // }
        }

        return min_hit;
    }
}
