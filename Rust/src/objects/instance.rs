use crate::{
    texture::texture::ImageTexture,
    vec3::{ray::Ray, vec3::Vec3},
};

use super::{
    aabb::{QuadAABB, AABB},
    materials::Material,
    maxf, minf,
    quad::Quad,
    sphere::Sphere,
    Object, NO_HIT,
};
#[derive(Debug, Clone)]
pub struct Instance {
    quads: Vec<Quad>,
    spheres: Vec<Sphere>,
    pub qaabb: QuadAABB,
    pub saabb: AABB,

    translation: Vec3,
    rotation: Vec3,
}

impl Instance {
    const PI2: f32 = 2.0 * std::f32::consts::PI;
    pub fn new_box(a: Vec3, b: Vec3, tex: ImageTexture, mat: Material) -> Self {
        let min = Vec3::new(minf(a.x, b.x), minf(a.y, b.y), minf(a.z, b.z));
        let max = Vec3::new(maxf(a.x, b.x), maxf(a.y, b.y), maxf(a.z, b.z));

        let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z - min.z);

        Instance::new_quads(vec![
            Quad::new(
                Vec3::new(min.x, min.y, max.z),
                dx,
                dy,
                mat,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            ),
            Quad::new(
                Vec3::new(max.x, min.y, max.z),
                -dz,
                dy,
                mat,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            ),
            Quad::new(
                Vec3::new(max.x, min.y, min.z),
                -dx,
                dy,
                mat,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            ),
            Quad::new(
                Vec3::new(min.x, min.y, min.z),
                dz,
                dy,
                mat,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            ),
            Quad::new(
                Vec3::new(min.x, max.y, max.z),
                dx,
                -dz,
                mat,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            ),
            Quad::new(
                Vec3::new(min.x, min.y, min.z),
                dx,
                dz,
                mat,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                tex.to_owned(),
            ),
        ])
    }
    pub fn new_sphere(spheres: Vec<Sphere>) -> Self {
        Self {
            spheres: spheres.clone(),
            saabb: AABB::new(spheres),
            quads: vec![],
            qaabb: QuadAABB::empty(),

            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    pub fn new_quads(quads: Vec<Quad>) -> Self {
        Self {
            spheres: vec![],
            saabb: AABB::empty(),
            quads: quads.to_owned(),
            qaabb: QuadAABB::new(quads),

            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
    pub fn new(spheres: Vec<Sphere>, quads: Vec<Quad>) -> Self {
        Self {
            spheres: spheres.clone(),
            saabb: AABB::new(spheres),
            quads: quads.to_owned(),
            qaabb: QuadAABB::new(quads),

            translation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            rotation: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }

    pub fn rotate(&mut self, rot: Vec3) {
        self.rotation += rot;
    }

    pub fn translate(&mut self, vec: Vec3) {
        self.translation += vec;
    }
    pub fn gett(&self) -> Vec3 {
        return self.translation.to_owned();
    }
}

impl Object for Instance {
    fn collide(&self, r: crate::vec3::ray::Ray) -> bool {
        self.collision_normal(r, 0.0001, 10000.0) != NO_HIT
    }

    fn collision_normal(&self, r: crate::vec3::ray::Ray, mint: f32, maxt: f32) -> super::Hit {
        //change to local
        let r = Ray::new_with_time(r.origin - self.translation, r.direction, r.time);

        //check
        let mut min_hit = NO_HIT;
        let s_hit = self.saabb.collision_normal(r, mint, maxt);
        let q_hit = self.qaabb.collision_normal(r, mint, maxt);
        for i in vec![s_hit, q_hit] {
            if i == NO_HIT {
                continue;
            }
            if min_hit == NO_HIT || min_hit > i {
                min_hit = i;
            }
        }

        // change to global
        min_hit.point += self.translation;
        min_hit
    }
}
