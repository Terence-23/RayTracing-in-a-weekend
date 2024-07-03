use std::sync::Arc;

use crate::{
    onb::ONB,
    quaternions::{Quaternion, ZERO_ROTATION},
    rotation::Rotation,
    vec3::{ray::Ray, vec3::Vec3},
    viewport::scene::Scene,
};

use super::{
    aabb::{maxf, minf, Interval, AABB},
    hit::Hit,
    material::Material,
    quad::Quad,
    texture::Texture,
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
    pub fn new_box(a: Vec3, b: Vec3, tex: Arc<dyn Texture>, mat: Arc<dyn Material>) -> Self {
        let min = Vec3::new(minf(a.x, b.x), minf(a.y, b.y), minf(a.z, b.z));
        let max = Vec3::new(maxf(a.x, b.x), maxf(a.y, b.y), maxf(a.z, b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        Instance::new(Arc::new([
            Arc::new(Quad::new(
                Vec3::new(min.x, min.y, max.z),
                dx,
                dy,
                mat.clone(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            )),
            Arc::new(Quad::new(
                Vec3::new(max.x, min.y, max.z),
                -dz,
                dy,
                mat.clone(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            )),
            Arc::new(Quad::new(
                Vec3::new(max.x, min.y, min.z),
                -dx,
                dy,
                mat.clone(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            )),
            Arc::new(Quad::new(
                Vec3::new(min.x, min.y, min.z),
                dz,
                dy,
                mat.clone(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            )),
            Arc::new(Quad::new(
                Vec3::new(min.x, max.y, max.z),
                dx,
                -dz,
                mat.clone(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            )),
            Arc::new(Quad::new(
                Vec3::new(min.x, min.y, min.z),
                dx,
                dz,
                mat.clone(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            )),
        ]))
    }
    pub fn get_aabb(&self) -> AABB {
        let mut vecs = Interval::intervals_to_bounding_vecs(self.x, self.y, self.z);
        vecs.0 += self.gett();
        vecs.1 += self.gett();
        let (x, y, z) = Interval::from_vecs(vecs.0, vecs.1);
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
    pub fn get_hit(&self, mut r: Ray, s: &Scene) -> Option<(Hit, Arc<dyn Object>)> {
        // eprintln!("instance_hit");
        // debug_assert!(r.direction.is_normal(), "dir is nan");
        let mut min_h = None;
        r.origin -= self.position;
        // debug_assert!(r.direction.is_normal(), "dir2 is nan");
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
        if let Some(mut hit) = min_h {
            hit.0.p = self.rotation.rotate(&hit.0.p);
            hit.0.p += self.position;

            // debug_assert!(hit.0.n.length2() > 1e-8);
            // let sn = hit.0.n;
            hit.0.n = self.rotation.rotate(&hit.0.n);
            // debug_assert!(hit.0.n.length2() > 1e-8, "{:?}, {:?}", self.rotation, sn);
            return Some(hit);
        }

        return None;
    }
}
