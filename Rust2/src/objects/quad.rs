use std::sync::Arc;

use crate::vec3::vec3::Vec3;

use super::{
    aabb::{maxf, minf, Interval},
    material::Material,
    texture::{ImageTexture, Texture},
    Object,
};

pub struct Quad {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat: Arc<dyn Material>,
    pub velocity: Vec3,
    pub texture: Arc<dyn Texture>,

    //Internals
    normal: Vec3,
    d: f32,
    w: Vec3,
}

impl Quad {
    pub fn new(
        origin: Vec3,
        u: Vec3,
        v: Vec3,
        mat: Arc<dyn Material>,
        velocity: Vec3,
        texture: Arc<dyn Texture>,
    ) -> Self {
        let n = u.cross(v);
        let normal = n.unit();
        Self {
            origin,
            u,
            v,
            mat,
            velocity,
            texture,
            normal,
            d: normal.dot(origin),
            w: n / n.dot(n),
        }
    }
}

impl Object for Quad {
    fn get_aabb(
        &self,
    ) -> (
        super::aabb::Interval,
        super::aabb::Interval,
        super::aabb::Interval,
    ) {
        let op_corner = self.origin + self.u + self.v;
        let v_corner = self.origin + self.v;
        let u_corner = self.origin + self.u;

        (
            Interval {
                min: minf(
                    minf(op_corner.x, v_corner.x),
                    minf(u_corner.x, self.origin.x),
                ),
                max: maxf(
                    maxf(op_corner.x, v_corner.x),
                    maxf(u_corner.x, self.origin.x),
                ),
            },
            Interval {
                min: minf(
                    minf(op_corner.y, v_corner.y),
                    minf(u_corner.y, self.origin.y),
                ),
                max: maxf(
                    maxf(op_corner.y, v_corner.y),
                    maxf(u_corner.y, self.origin.y),
                ),
            },
            Interval {
                min: minf(
                    minf(op_corner.z, v_corner.z),
                    minf(u_corner.z, self.origin.z),
                ),
                max: maxf(
                    maxf(op_corner.z, v_corner.z),
                    maxf(u_corner.z, self.origin.z),
                ),
            },
        )
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
        if alfa < 0.0 || alfa > 1.0 || beta < 0.0 || beta > 1.0 {
            // eprintln!("Out of quad");
            // eprintln!("alfa: {alfa}, beta: {beta}");
            return None;
        }
        // eprintln!("Hit");
        Some(super::Hit {
            t: t,
            n: self.normal,
            p: point,
            r,
        })
    }

    fn reflect(&self, h: &super::hit::Hit) -> super::material::ReflectResult {
        self.mat.on_hit(h)
    }

    fn color(&self, h: &super::hit::Hit) -> super::texture::ColorResult {
        let planar = h.p - self.origin;
        let alfa = self.w.dot(planar.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar));
        // eprintln!("color: {:?}", self.texture.color_at(alfa, beta).multiplied);
        self.texture.color_at(alfa, beta)
    }
}

mod tests {
    use std::sync::Arc;

    use crate::{
        objects::{
            instance::Instance, material::LAMBERTIAN, quad::Quad, texture::ConstColorTexture,
        },
        vec3::vec3::Vec3,
        viewport::{camera::Camera, ray_color::ray_color, scene::Scene, Viewport},
    };

    #[test]
    fn quad_test() -> image::ImageResult<()> {
        let samples = 100;
        // let _spheres = Arc::new([Sphere::new(
        //     Vec3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: -1.0,
        //     },
        //     0.45,
        //     Some(Vec3::new(0.95, 0.95, 0.95)),
        //     LAMBERTIAN.to_owned(),
        // )]);
        let quads = Instance::new(Arc::new([
            //Red
            Arc::new(Quad::new(
                Vec3 {
                    x: -3.0,
                    y: -2.0,
                    z: 5.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                LAMBERTIAN.to_owned(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Arc::new(ConstColorTexture::new(
                    Vec3 {
                        x: 1.0,
                        y: 0.2,
                        z: 0.2,
                    },
                    Vec3::zero(),
                )),
            )),
            //Green
            Arc::new(Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                LAMBERTIAN.to_owned(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Arc::new(ConstColorTexture::new(
                    Vec3 {
                        x: 0.2,
                        y: 1.0,
                        z: 0.2,
                    },
                    Vec3::zero(),
                )),
            )),
            //Blue
            Arc::new(Quad::new(
                Vec3 {
                    x: 3.0,
                    y: -2.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 4.0,
                    z: 0.0,
                },
                LAMBERTIAN.to_owned(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Arc::new(ConstColorTexture::new(
                    Vec3 {
                        z: 1.0,
                        y: 0.2,
                        x: 0.2,
                    },
                    Vec3::zero(),
                )),
            )),
            //Orange
            Arc::new(Quad::new(
                Vec3 {
                    x: -2.0,
                    y: 3.0,
                    z: 1.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 4.0,
                },
                LAMBERTIAN.to_owned(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Arc::new(ConstColorTexture::new(
                    Vec3 {
                        x: 1.0,
                        y: 0.5,
                        z: 0.,
                    },
                    Vec3::zero(),
                )),
            )),
            //Teal
            Arc::new(Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -3.0,
                    z: 5.0,
                },
                Vec3 {
                    x: 4.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -4.0,
                },
                LAMBERTIAN.to_owned(),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Arc::new(ConstColorTexture::new(
                    Vec3 {
                        x: 0.2,
                        y: 0.8,
                        z: 0.8,
                    },
                    Vec3::zero(),
                )),
            )),
        ]));

        // let scene = Scene::new(spheres, quads.to_owned());
        let scene = Scene::new(vec![quads], 0.001, 1000.);
        const WIDTH: usize = 800;
        const HEIGHT: usize = 600;
        let camera = Camera::new(
            WIDTH as f32 / HEIGHT as f32,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 8.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            100.0,
            0.,
        );
        let viewport = Viewport::new(
            camera,
            scene,
            &ray_color,
            WIDTH,
            HEIGHT,
            samples,
            10,
            Vec3 {
                x: 0.9,
                y: 0.9,
                z: 0.9,
            },
            1.0,
        );
        eprintln!("Running");

        let img = viewport.render();
        img.save("test_out/quad_test.png")
    }

    #[test]
    fn simple_test() -> image::ImageResult<()> {
        //let spheres = vec![
        // Sphere::new_with_texture(
        // Vec3 {
        //     x: -0.5,
        //     y: 0.0,
        //     z: -1.0,
        // },
        // 0.5,
        // None,
        // Some(LAMBERTIAN.to_owned()),
        // ImageTexture::from_color(Rgb([1.0, 0.3, 1.0])),
        // )
        // ];
        let quads = Instance::new(Arc::new([Arc::new(Quad::new(
            Vec3 {
                x: -2.0,
                y: -2.0,
                z: 0.0,
            },
            Vec3 {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 4.0,
                z: 0.0,
            },
            LAMBERTIAN.to_owned(),
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Arc::new(ConstColorTexture::new(
                Vec3 {
                    x: 0.2,
                    y: 1.0,
                    z: 0.2,
                },
                Vec3::zero(),
            )),
        ))]));
        let scene = Scene::new(vec![quads], 0.001, 10000.0);
        let samples = 100;
        const WIDTH: usize = 80;
        const HEIGHT: usize = 60;
        let camera = Camera::new(
            WIDTH as f32 / HEIGHT as f32,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 3.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            80.0,
            0.,
        );
        let viewport = Viewport::new(
            camera,
            scene,
            &ray_color,
            WIDTH,
            HEIGHT,
            samples,
            10,
            Vec3 {
                x: 0.9,
                y: 0.4,
                z: 0.3,
            },
            1.0,
        );
        eprintln!("Running");

        let img = viewport.render();
        img.save("test_out/simple_quad_test.png")
    }
}
