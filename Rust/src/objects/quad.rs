use super::{materials::Material, Object, NO_HIT};
use crate::{
    texture::texture::{ImageTexture, Texture},
    vec3::vec3::Vec3,
};

#[derive(Debug, Clone)]
pub struct Quad {
    pub origin: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat: Material,
    pub velocity: Vec3,
    pub texture: ImageTexture,

    //Internals
    normal: Vec3,
    d: f32,
    w: Vec3,
}

impl PartialEq for Quad {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin
            && self.u == other.u
            && self.v == other.v
            && self.mat == other.mat
            && self.velocity == other.velocity
    }
}

impl Object for Quad {
    fn collide(&self, r: crate::vec3::ray::Ray) -> bool {
        self.collision_normal(r, 0.0001, 10000.0) != NO_HIT
    }

    fn collision_normal(&self, r: crate::vec3::ray::Ray, mint: f32, maxt: f32) -> super::Hit {
        let denominator = self.normal.dot(r.direction);
        if denominator.abs() <= 1e-8 {
            // eprintln!("parallel");
            return NO_HIT;
        }
        let t = (self.d - self.normal.dot(r.origin)) / denominator;
        if t < mint || t > maxt {
            // eprintln!("Out of range");
            // dbg!(t);
            // dbg!(r);
            // dbg!(self);
            return NO_HIT;
        }
        let point = r.at(t);
        let planar = point - self.origin;

        let alfa = self.w.dot(planar.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar));
        if alfa < 0.0 || alfa > 1.0 || beta < 0.0 || beta > 1.0 {
            // eprintln!("Out of quad");
            // eprintln!("alfa: {alfa}, beta: {beta}");
            return NO_HIT;
        }

        super::Hit {
            t: t,
            normal: self.normal,
            point: point,
            col_mod: self.texture.color_at(
                if alfa != 1.0 {
                    (alfa * self.texture.row as f32).floor() as usize
                } else {
                    self.texture.row - 1
                },
                if beta != 1.0 {
                    (beta * self.texture.col as f32).floor() as usize
                } else {
                    self.texture.col - 1
                },
                point,
            ),
            mat: self.mat,
        }
    }
}
#[allow(dead_code)]
impl Quad {
    pub fn new(
        origin: Vec3,
        u: Vec3,
        v: Vec3,
        mat: Material,
        velocity: Vec3,
        texture: ImageTexture,
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

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::{
        objects::{materials::SCATTER_M, sphere::Sphere},
        vec3::ray::Ray,
        viewport::{Scene, Viewport},
        write_img::img_writer::write_img_f32,
    };

    fn ray_color_d(r: Ray, scene: &Scene, depth: usize) -> Rgb<f32> {
        // eprintln!("D: {}", depth);
        if depth < 1 {
            return Rgb([0.0, 0.0, 0.0]);
        }
        let mint = 0.001;
        let maxt = 1000.0;

        let hit = scene.collision_normal(r, mint, maxt);

        if hit != NO_HIT {
            // eprintln!("Hit: {:?}", hit );
            let cm = hit.col_mod;
            let front = if r.direction.dot(hit.normal) > 0.0 {
                false
            } else {
                true
            };
            let (mut next, _) = hit.mat.on_hit(hit, r);
            if next.direction.close_to_zero() {
                next.direction = if front { hit.normal } else { hit.normal * -1.0 };
            }
            // println!("depth: {}", depth);
            return (Vec3::from_rgb(ray_color_d(next, scene, depth - 1)) * cm).to_rgb();
        }
        // eprintln!("Sky");
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        return Rgb([(1.0 - t) + t * 0.5, (1 as f32 - t) + t * 0.7, 1.0]);
    }

    use super::*;

    #[test]
    fn quad_test() {
        let samples = 100;
        let _spheres = vec![Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.45,
            Some(Vec3::new(0.95, 0.95, 0.95)),
            Some(SCATTER_M),
        )];
        let quads = vec![
            //Red
            Quad::new(
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
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [1.0, 0.2, 0.2] }),
            ),
            //Green
            Quad::new(
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
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 1.0, 0.2] }),
            ),
            //Blue
            Quad::new(
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
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 0.2, 1.0] }),
            ),
            //Orange
            Quad::new(
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
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [1.0, 0.5, 0.0] }),
            ),
            //Teal
            Quad::new(
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
                SCATTER_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 0.8, 0.8] }),
            ),
        ];
        // let scene = Scene::new(spheres, quads.to_owned());
        let scene = Scene::new_quad(quads);
        let viewport = Viewport::new_from_res(
            400,
            400,
            samples,
            10,
            2.0,
            Some(80.0),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 9.0,
            }),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }),
            None,
            Some("Quad test".to_string()),
            None,
        );
        eprintln!("Running");

        let img = viewport.render(&ray_color_d, scene);

        write_img_f32(&img, "out/quad_test.png".to_string());
    }

    #[test]
    fn simple_test() {
        let spheres = vec![
            // Sphere::new_with_texture(
            // Vec3 {
            //     x: -0.5,
            //     y: 0.0,
            //     z: -1.0,
            // },
            // 0.5,
            // None,
            // Some(SCATTER_M),
            // ImageTexture::from_color(Rgb([1.0, 0.3, 1.0])),
            // )
        ];
        let quads = vec![Quad::new(
            Vec3 {
                x: -1.0,
                y: -0.5,
                z: -2.0,
            },
            Vec3 {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            SCATTER_M,
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            ImageTexture::from_color(Rgb([0.20, 1.0, 0.20])),
        )];
        let scene = Scene::new(spheres, quads, vec![]);
        let samples = 100;
        let viewport = Viewport::new_from_res(
            200,
            200,
            samples,
            10,
            2.0,
            Some(80.0),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }),
            None,
            Some("Quad test".to_string()),
            None,
        );
        eprintln!("Running");

        let img = viewport.render(&ray_color_d, scene);

        write_img_f32(&img, "out/simple_quad_test.png".to_string());
    }
}
