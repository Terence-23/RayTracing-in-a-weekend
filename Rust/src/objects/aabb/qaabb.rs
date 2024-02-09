use rand::random;

use crate::{
    objects::{maxf, minf, quad::Quad, Hit, Interval, Object},
    vec3::ray::Ray,
};

use super::Axis;

#[derive(Debug, Clone)]
pub struct QuadAABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
    quads: Vec<Quad>,
    aabbs: Vec<QuadAABB>,
}

impl From<Quad> for QuadAABB {
    fn from(quad: Quad) -> Self {
        let op_corner = quad.origin + quad.u + quad.v;
        let v_corner = quad.origin + quad.v;
        let u_corner = quad.origin + quad.u;
        let mut aabb = QuadAABB {
            x: Interval {
                min: minf(
                    minf(op_corner.x, v_corner.x),
                    minf(u_corner.x, quad.origin.x),
                ),
                max: maxf(
                    maxf(op_corner.x, v_corner.x),
                    maxf(u_corner.x, quad.origin.x),
                ),
            },
            y: Interval {
                min: minf(
                    minf(op_corner.y, v_corner.y),
                    minf(u_corner.y, quad.origin.y),
                ),
                max: maxf(
                    maxf(op_corner.y, v_corner.y),
                    maxf(u_corner.y, quad.origin.y),
                ),
            },
            z: Interval {
                min: minf(
                    minf(op_corner.z, v_corner.z),
                    minf(u_corner.z, quad.origin.z),
                ),
                max: maxf(
                    maxf(op_corner.z, v_corner.z),
                    maxf(u_corner.z, quad.origin.z),
                ),
            },
            quads: vec![quad.clone()],
            aabbs: vec![],
        };
        aabb.pad();
        aabb
    }
}
impl From<&Quad> for QuadAABB {
    fn from(quad: &Quad) -> Self {
        let op_corner = quad.origin + quad.u + quad.v;
        let v_corner = quad.origin + quad.v;
        let u_corner = quad.origin + quad.u;
        let mut aabb = QuadAABB {
            x: Interval {
                min: minf(
                    minf(op_corner.x, v_corner.x),
                    minf(u_corner.x, quad.origin.x),
                ),
                max: maxf(
                    maxf(op_corner.x, v_corner.x),
                    maxf(u_corner.x, quad.origin.x),
                ),
            },
            y: Interval {
                min: minf(
                    minf(op_corner.y, v_corner.y),
                    minf(u_corner.y, quad.origin.y),
                ),
                max: maxf(
                    maxf(op_corner.y, v_corner.y),
                    maxf(u_corner.y, quad.origin.y),
                ),
            },
            z: Interval {
                min: minf(
                    minf(op_corner.z, v_corner.z),
                    minf(u_corner.z, quad.origin.z),
                ),
                max: maxf(
                    maxf(op_corner.z, v_corner.z),
                    maxf(u_corner.z, quad.origin.z),
                ),
            },
            quads: vec![quad.clone()],
            aabbs: vec![],
        };
        aabb.pad();
        aabb
    }
}
#[allow(dead_code)]
impl QuadAABB {
    const MIN_SIZE_DIR: f32 = 0.00001;
    pub fn pad(&mut self) {
        if self.x.max - self.x.min < Self::MIN_SIZE_DIR {
            let center = (self.x.max - self.x.min) / 2.0 + self.x.min;
            self.x.min = center - 0.5 * Self::MIN_SIZE_DIR;
            self.x.max = center + 0.5 * Self::MIN_SIZE_DIR;
        }
        let y = &mut self.y;
        if y.max - y.min < Self::MIN_SIZE_DIR {
            let center = (y.max - y.min) / 2.0 + y.min;
            y.min = center - 0.5 * Self::MIN_SIZE_DIR;
            y.max = center + 0.5 * Self::MIN_SIZE_DIR;
        }
        let z = &mut self.z;
        if z.max - z.min < Self::MIN_SIZE_DIR {
            let center = (z.max - z.min) / 2.0 + z.min;
            z.min = center - 0.5 * Self::MIN_SIZE_DIR;
            z.max = center + 0.5 * Self::MIN_SIZE_DIR;
        }
    }
    pub fn new(mut quads: Vec<Quad>) -> Self {
        if quads.len() == 0 {
            return Self::empty();
        }
        if quads.len() == 1 {
            return (&quads[0]).into();
        }
        let axis = random::<Axis>();
        match axis {
            Axis::X => quads.sort_unstable_by(|s, oth| {
                (QuadAABB::from(s).x.min).total_cmp(&(QuadAABB::from(oth).x.min))
            }),
            Axis::Y => quads.sort_unstable_by(|s, oth| {
                (QuadAABB::from(s).y.min).total_cmp(&(QuadAABB::from(oth).y.min))
            }),
            Axis::Z => quads.sort_unstable_by(|s, oth| {
                (QuadAABB::from(s).z.min).total_cmp(&(QuadAABB::from(oth).z.min))
            }),
        }
        return Self::new_from_sorted(quads);
    }
    fn new_from_sorted(quads: Vec<Quad>) -> Self {
        let len = quads.len() / 2;
        let quads1 = quads[0..len].to_vec();
        let quads2 = quads[len..].to_vec();

        let aabb1 = Self::new(quads1);
        let aabb2 = Self::new(quads2);

        return QuadAABB {
            x: aabb1.x + aabb2.x,
            y: aabb1.y + aabb2.y,
            z: aabb1.z + aabb2.z,
            quads: quads,
            aabbs: vec![aabb1, aabb2],
        };
    }
    pub fn volume(&self) -> f32 {
        (self.x.max - self.x.min) * (self.y.max - self.y.min) * (self.z.max - self.z.min)
    }
    pub fn empty() -> Self {
        QuadAABB {
            x: Interval { min: 0.0, max: 0.0 },
            y: Interval { min: 0.0, max: 0.0 },
            z: Interval { min: 0.0, max: 0.0 },
            quads: vec![],
            aabbs: vec![],
        }
    }
}

impl Object for QuadAABB {
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
        if self.quads.is_empty() && self.aabbs.is_empty() {
            // eprintln!("empty; qaabb.rs:199");
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
            // eprintln!("min > max");
            // dbg!(min);
            // dbg!(max);
            // dbg!(x_hit, y_hit, z_hit);

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
            // eprintln!("quad intersect");
            for i in self.quads[..]
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

            // if min_hit == NO_HIT {
            //     eprintln!("NO_HIT");
            // }
        }

        return min_hit;
    }
}
