use rand::{distributions::{Distribution, Standard}, Rng, random};

use crate::vec3::ray::Ray;

use super::{Object, sphere::Sphere, maxf, minf, Hit, NO_HIT, Interval};
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

enum Axis{
    X,
    Y,
    Z
}
impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) { // rand 0.8
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
    spheres: Vec<Sphere>,
    aabbs: Vec<AABB> 
}
#[allow(dead_code)]
impl AABB{
    pub fn new(mut spheres: Vec<Sphere>, ) -> Self{
        if spheres.len() == 1{
            return AABB{
                spheres: spheres.to_owned(),
                x: Interval::new(spheres[0].origin.x - spheres[0].radius, spheres[0].origin.x + spheres[0].radius),
                y: Interval::new(spheres[0].origin.y - spheres[0].radius, spheres[0].origin.y + spheres[0].radius),
                z: Interval::new(spheres[0].origin.z - spheres[0].radius, spheres[0].origin.z + spheres[0].radius),
                aabbs: vec![],
            }
        }
        let axis = random::<Axis>();
        match axis {
            Axis::X => spheres.sort_unstable_by(|s, oth| (s.origin.x + s.radius).total_cmp(&(oth.origin.x + oth.radius))),
            Axis::Y => spheres.sort_unstable_by(|s, oth| (s.origin.y + s.radius).total_cmp(&(oth.origin.y + oth.radius))),
            Axis::Z => spheres.sort_unstable_by(|s, oth| (s.origin.z + s.radius).total_cmp(&(oth.origin.z + oth.radius)))
        }
        return Self::new_from_sorted(spheres)
    }
    fn new_from_sorted(spheres: Vec<Sphere>) -> Self{
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
            aabbs: vec![aabb1, aabb2] }
    }
    pub fn volume(&self) -> f32{
        (self.x.max - self.x.min) * (self.y.max - self.y.min) * (self.z.max - self.z.min)
    }
}

impl Object for AABB{
fn collide(&self, r: Ray) -> bool {
    let x_hit = match self.x.intersect(r.direction.x, r.origin.x){
        Some(n) => n,
        None => return false
    };
    let y_hit = match self.y.intersect(r.direction.y, r.origin.y){
        Some(n) => n,
        None => return false
    };
    let z_hit = match self.z.intersect(r.direction.z, r.origin.z){
        Some(n) => n,
        None => return false
    };
    let min = maxf(maxf(x_hit.min, y_hit.min), z_hit.min);
    let max = minf(minf(x_hit.max, y_hit.max), z_hit.max);
    min < max
}

fn collision_normal(&self, r: Ray, mint:f32, maxt:f32) -> Hit {
    let x_hit = match self.x.intersect(r.direction.x, r.origin.x){
        Some(n) => n,
        None => return NO_HIT
    };
    let y_hit = match self.y.intersect(r.direction.y, r.origin.y){
        Some(n) => n,
        None => return NO_HIT
    };
    let z_hit = match self.z.intersect(r.direction.z, r.origin.z){
        Some(n) => n,
        None => return NO_HIT
    };
    let min = maxf(maxf(x_hit.min, y_hit.min), maxf(z_hit.min, mint));
    let max = minf(minf(x_hit.max, y_hit.max), minf(z_hit.max, maxt));
    
    if min > max{
        return NO_HIT;
    }
    let mut min_hit = NO_HIT;
    if self.aabbs.len() > 0{
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

    }
    else{
        for i in self.spheres[..]
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
    }

    return min_hit;
}
}
