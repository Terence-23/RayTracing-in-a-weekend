use std::sync::Arc;

use crate::{
    onb::ONB,
    quaternions::{Quaternion, ZERO_ROTATION},
    rotation::Rotation,
    vec3::{ray::Ray, vec3::Vec3},
    viewport::scene::Scene,
};

use super::{
    aabb::{Interval, AABB},
    hit::Hit,
    Object,
};

#[derive(Clone)]
pub struct Instance {
    pub position: Vec3,
    rotation: Quaternion,
    onb: ONB,
    objects: Arc<[Arc<dyn Object>]>,
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Instance {
    pub fn rotate(&mut self, rot: impl Rotation) {
        self.rotation *= rot.into();
    }
    pub fn reset_rotation(&mut self) {
        self.rotation = ZERO_ROTATION;
    }
    pub fn getr(&self) -> Quaternion {
        return self.rotation.to_owned();
    }

    pub fn translate(&mut self, vec: Vec3) {
        self.position += vec;
    }
    pub fn gett(&self) -> Vec3 {
        return self.position.to_owned();
    }

    pub fn new(objects: Arc<[Arc<dyn Object>]>) -> Self {
        if objects.len() == 0 {
            return Self::empty();
        }

        let mut intervals = objects[0].get_aabb();
        for o in &objects[1..] {
            let ni = o.get_aabb();
            intervals.0 += ni.0;
            intervals.1 += ni.1;
            intervals.2 += ni.2;
        }
        Self {
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Quaternion {
                w: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            onb: ONB::new(),
            objects,
            x: intervals.0,
            y: intervals.1,
            z: intervals.2,
        }
    }
    pub fn get_aabb(&self) -> AABB {
        let vecs = Interval::intervals_to_bounding_vecs(self.x, self.y, self.z);
        let (x, y, z) =
            Interval::from_vecs(self.onb.from_local(vecs.0), self.onb.from_local(vecs.1));
        return AABB {
            x,
            y,
            z,
            instances: vec![self.to_owned()],
            aabbs: vec![],
        };
    }

    fn empty() -> Instance {
        Instance {
            position: Vec3::new(0.0, 0.0, 0.0),
            onb: ONB::new(),
            rotation: Quaternion {
                w: 1.0,
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            objects: Arc::new([]),
            x: Interval::new(0.0, 0.0),
            y: Interval::new(0.0, 0.0),
            z: Interval::new(0.0, 0.0),
        }
    }
    pub fn get_hit(&self, r: Ray, s: &Scene) -> Option<(Hit, Arc<dyn Object>)> {
        let mut min_h = None;
        for (i, h) in self
            .objects
            .iter()
            .map(|i| (i, i.get_hit(r, s.mint, s.maxt)))
        {
            if h.is_none() {
                continue;
            }
            if min_h.is_none() {
                min_h = Some((h.unwrap(), i.to_owned()));
                continue;
            }
            let hit = h.unwrap();
            if hit < min_h.clone().unwrap().0 {
                min_h = Some((hit, i.to_owned()));
            }
        }
        // if min_h.is_some() {
        //     let h = min_h.to_owned().unwrap().0;
        //     eprintln!("We Hit: {:?}\n dist to sphere_origin: {}", h, h.p.length())
        // }

        return min_h;
    }
}
