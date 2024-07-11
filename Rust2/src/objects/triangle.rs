use std::sync::Arc;

use crate::vec3::{ray::Ray, vec3::Vec3};

use super::{
    aabb::{maxf, minf, Interval},
    material::Material,
    texture::Texture,
    Object,
};

pub struct Triangle {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat: Arc<dyn Material>,
    // pub velocity: Vec3,
    pub texture: Arc<dyn Texture>,

    //Internals
    normal: Vec3,
    d: f32,
    w: Vec3,
}

impl Triangle {
    const MIN_AABB_WIDTH: f32 = 0.005;
    pub fn new(
        origin: Vec3,
        u: Vec3,
        v: Vec3,
        mat: Arc<dyn Material>,
        texture: Arc<dyn Texture>,
    ) -> Self {
        let n = u.cross(v);
        let normal = n.unit();
        Self {
            origin,
            u,
            v,
            mat,
            // velocity,
            texture,
            normal,
            d: normal.dot(origin),
            w: n / n.dot(n),
        }
    }
}

impl Object for Triangle {
    fn get_aabb(
        &self,
    ) -> (
        super::aabb::Interval,
        super::aabb::Interval,
        super::aabb::Interval,
    ) {
        let v_corner = self.origin + self.v;
        let u_corner = self.origin + self.u;
        let mut x = Interval {
            min: minf(v_corner.x, minf(u_corner.x, self.origin.x)),
            max: maxf(v_corner.x, maxf(u_corner.x, self.origin.x)),
        };
        if x.max - x.min < Triangle::MIN_AABB_WIDTH {
            let c = 0.5 * (x.max + x.min);
            x.max = c + Triangle::MIN_AABB_WIDTH * 0.5;
            x.min = c - Triangle::MIN_AABB_WIDTH * 0.5;
        }
        let mut y = Interval {
            min: minf(v_corner.y, minf(u_corner.y, self.origin.y)),
            max: maxf(v_corner.y, maxf(u_corner.y, self.origin.y)),
        };
        if y.max - y.min < Triangle::MIN_AABB_WIDTH {
            let c = 0.5 * (y.max + y.min);
            y.max = c + Triangle::MIN_AABB_WIDTH * 0.5;
            y.min = c - Triangle::MIN_AABB_WIDTH * 0.5;
        }
        let mut z = Interval {
            min: minf(v_corner.z, minf(u_corner.z, self.origin.z)),
            max: maxf(v_corner.z, maxf(u_corner.z, self.origin.z)),
        };
        if z.max - z.min < Triangle::MIN_AABB_WIDTH {
            let c = 0.5 * (z.max + z.min);
            z.max = c + Triangle::MIN_AABB_WIDTH * 0.5;
            z.min = c - Triangle::MIN_AABB_WIDTH * 0.5;
        }
        (x, y, z)
    }

    fn get_hit(&self, r: crate::vec3::ray::Ray, mint: f32, maxt: f32) -> Option<super::hit::Hit> {
        // eprintln!("Get hit");
        let denominator = self.normal.dot(r.direction);
        if denominator.abs() <= 1e-8 {
            // eprintln!("parallel");
            return None;
        }
        let t = (self.d - self.normal.dot(r.origin)) / denominator;
        if t < mint || t > maxt {
            // eprintln!("Out of range");
            // dbg!(t);
            // dbg!(r);
            // dbg!(self);
            return None;
        }
        let point = r.at(t);
        let planar = point - self.origin;

        let alfa = self.w.dot(planar.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar));
        if alfa < 0.0 || beta < 0.0 || alfa + beta > 1.0 {
            // eprintln!("Out of quad");
            // eprintln!("alfa: {alfa}, beta: {beta}");
            return None;
        }
        // eprintln!("Hit");
        // debug_assert!(self.normal.length2() > 1e-10);
        Some(super::Hit {
            t: t,
            n: self.normal,
            p: point,
            r,
        })
    }

    fn reflect(&self, h: &super::hit::Hit) -> Ray {
        self.mat.on_hit(h)
    }

    fn color(&self, h: &super::hit::Hit) -> super::texture::ColorResult {
        let planar = h.p - self.origin;
        let alfa = self.w.dot(planar.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar));
        // eprintln!("color: {:?}", self.texture.color_at(alfa, beta).multiplied);
        self.texture.color_at(alfa, beta)
    }

    fn generator_pdf(&self, h: &super::hit::Hit, r: &Ray) -> f32 {
        self.mat.generator_pdf(h, r)
    }

    fn material_pdf(&self, h: &super::hit::Hit, r: &Ray) -> f32 {
        self.mat.material_pdf(h, r)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use image::ImageResult;

    use crate::{
        objects::{
            instance::Instance, material::LAMBERTIAN, texture::ConstColorTexture,
            triangle::Triangle,
        },
        vec3::vec3::Vec3,
        viewport::{camera::Camera, ray_color::ray_color, scene::Scene, Viewport},
    };

    #[test]
    fn triangle_test() -> ImageResult<()> {
        const WIDTH: usize = 400;
        const HEIGHT: usize = 300;
        const SAMPLES: usize = 25;
        const DEPTH: usize = 2;
        const BIASED_WEIGHT: f32 = 100.;
        let cam = Camera::new(
            WIDTH as f32 / HEIGHT as f32,
            Vec3::ZERO,
            Vec3::UP,
            Vec3::FORWARD,
            50.0,
            0.0,
        );
        let triangles = Instance::new(Arc::new([Arc::new(Triangle::new(
            Vec3::LEFT + Vec3::DOWN + Vec3::FORWARD * 3.,
            Vec3::UP * 2. + Vec3::RIGHT,
            Vec3::RIGHT * 2. + Vec3::UP,
            LAMBERTIAN.clone(),
            Arc::new(ConstColorTexture::new(Vec3::WHITE * 0.5, Vec3::BLACK)),
        ))]));

        let scene = Scene::new(vec![triangles], 0.0001, 10000.);

        let vp = Viewport::new(
            cam.clone(),
            scene.clone(),
            &ray_color,
            WIDTH,
            HEIGHT,
            SAMPLES,
            DEPTH,
            Vec3::WHITE * 0.6,
            1.0,
        );

        vp.render().save("test_out/triangle_test.png")
    }
}
