use std::iter::zip;

use rand::{thread_rng, Rng};

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
    Object,
};

type DistFn = dyn Fn(f32) -> f32 + Sync;

fn surface(_: f32) -> f32 {
    -1.0
}
pub fn const_density(d: f32) -> f32 {
    thread_rng().gen::<f32>().ln() / -d
}

#[derive(Clone)]
pub struct Instance {
    quads: Vec<Quad>,
    spheres: Vec<Sphere>,
    pub qaabb: QuadAABB,
    pub saabb: AABB,
    translation: Vec3,
    rotation: Vec3,

    pub dist_fn: &'static DistFn,
    pub density: f32,
}

impl std::fmt::Debug for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Instance")
            .field("quads", &self.quads)
            .field("spheres", &self.spheres)
            .field("qaabb", &self.qaabb)
            .field("saabb", &self.saabb)
            .field("translation", &self.translation)
            .field("rotation", &self.rotation)
            .finish()
    }
}
impl PartialEq for Instance {
    fn eq(&self, other: &Self) -> bool {
        if self.rotation != other.rotation || self.translation != other.translation {
            eprintln!("diff rot/trans");
            eprintln!(
                "{:?}{:?}\n{:?}{:?}",
                self.rotation, self.translation, other.rotation, self.translation
            );
            return false;
        }
        for (i, o) in zip(self.spheres.to_owned(), other.spheres.to_owned()) {
            if i != o {
                eprintln!("diff spheres");
                return false;
            }
        }
        for (i, o) in zip(self.quads.to_owned(), other.quads.to_owned()) {
            if i != o {
                eprintln!("diff quads");
                return false;
            }
        }
        eprintln!("Instance same");
        return true;
    }
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
            dist_fn: &surface,
            density: 0.0,
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
            dist_fn: &surface,
            density: 0.0,
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
            dist_fn: &surface,
            density: 0.0,
        }
    }

    pub fn rotate(&mut self, rot: Vec3) {
        self.rotation += rot;
    }
    pub fn getr(&self) -> Vec3 {
        return self.rotation.to_owned();
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
        self.collision_normal(r, 0.0001, 10000.0) != None
    }

    fn collision_normal(
        &self,
        r: crate::vec3::ray::Ray,
        mint: f32,
        maxt: f32,
    ) -> Option<super::Hit> {
        //change to local
        let mut r = Ray::new_with_time(r.origin - self.translation, r.direction, r.time)
            .rotated(-self.rotation);

        //check
        let mut min_hit = None;
        let s_hit = self.saabb.collision_normal(r, mint, maxt);
        let q_hit = self.qaabb.collision_normal(r, mint, maxt);
        for i in vec![s_hit, q_hit] {
            if i == None {
                continue;
            }
            if min_hit == None || min_hit > i {
                min_hit = i;
            }
        }

        // change to global

        // if min_hit != None {
        //     dbg!(min_hit);
        // } else {
        //     eprintln!("NO_HIT");
        // }
        if let Some(mut hit) = min_hit {
            let distance = (self.dist_fn)(self.density);
            if distance >= 0.0 {
                r.origin = hit.point + r.direction * distance;
                let mut min_hit = None;
                let s_hit = self.saabb.collision_normal(r, mint, maxt);
                let q_hit = self.qaabb.collision_normal(r, mint, maxt);
                for i in vec![s_hit, q_hit] {
                    if i == None {
                        continue;
                    }
                    if min_hit == None || min_hit > i {
                        min_hit = i;
                    }
                }
                match min_hit {
                    Some(_) => {
                        hit.point = r.origin;
                        hit.normal = Vec3::random_unit_vec();
                    }
                    None => return None,
                }
            }
            hit.point.rotate(self.rotation);
            hit.point += self.translation;

            hit.normal.rotate(self.rotation);
            return Some(hit);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::{
        objects::{
            aabb::IAABB,
            instance::{const_density, Instance},
            materials::{Material, METALLIC_M, SCATTER_M},
            quad::Quad,
            sphere::Sphere,
        },
        texture::texture::ImageTexture,
        vec3::vec3::Vec3,
        viewport::{async_render, ray_color::ray_color_bg_color, Scene, Viewport},
        write_img::img_writer::write_img_f32,
    };

    #[test]
    fn box_test() {
        const PI: f32 = std::f32::consts::PI;
        let samples = 100;
        let _spheres = vec![Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            0.4,
            Some(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            Some(Material::new_emmiting(
                1.0,
                0.0,
                0.0,
                Vec3 {
                    x: 2.0,
                    y: 0.5,
                    z: 2.0,
                },
            )),
        )];

        let quads = vec![
            //Red
            Quad::new(
                Vec3 {
                    x: -2.0,
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
                METALLIC_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb {
                    0: [0.85, 0.85, 0.85],
                }),
            ),
            //Light
            // Quad::new(
            //     Vec3 {
            //         x: -0.5,
            //         y: -0.5,
            //         z: 1.001,
            //     },
            //     Vec3 {
            //         x: 1.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     Vec3 {
            //         x: 0.0,
            //         y: 1.0,
            //         z: 0.0,
            //     },
            //     Material::new_emmiting(
            //         0.0,
            //         0.0,
            //         1.0,
            //         Vec3 {
            //             x: 4.0,
            //             y: 2.0,
            //             z: 4.0,
            //         },
            //     ),
            //     Vec3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     ImageTexture::from_color(Rgb { 0: [1.0, 1.0, 1.0] }),
            // ),
            //Blue
            Quad::new(
                Vec3 {
                    x: 2.0,
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
                    y: 2.0,
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
                    y: -2.0,
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
            //Green
            Quad::new(
                Vec3 {
                    x: -0.5,
                    y: -0.5,
                    z: 1.0,
                },
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                Material::new_emmiting(
                    0.0,
                    0.0,
                    1.0,
                    Vec3 {
                        x: 4.0,
                        y: 4.0,
                        z: 4.0,
                    },
                ),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 1.0, 0.2] }),
            ),
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 0.999,
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
                Material::new_emmiting(
                    0.0,
                    0.0,
                    1.0,
                    Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                ),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 1.0, 0.2] }),
            ),
        ];
        // let scene = Scene::new(spheres, quads.to_owned());
        let mut instance = Instance::new_box(
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, 0.5),
            ImageTexture::from_color(Vec3::new(0.0, 0.5, 0.0).to_rgb()),
            SCATTER_M,
        );
        instance.translate(Vec3 {
            x: -0.0,
            y: -1.5,
            z: 2.0,
        });
        let scene = Scene::new(vec![], quads.to_owned(), vec![instance.to_owned()]);
        // instance.translate(Vec3 {
        //     x: 1.0,
        //     y: 1.0,
        //     z: 0.0,
        // });
        instance.rotate(Vec3 {
            x: 0.0,
            y: PI / 4.0,
            z: 0.0,
        });
        let aabb = IAABB::from(&instance);
        dbg!(aabb.x, aabb.y, aabb.z);
        let scene2 = Scene::new(vec![], quads.to_owned(), vec![instance.to_owned()]);
        // assert_eq!(scene, scene2);
        let viewport = Viewport::new_from_res(
            400,
            400,
            samples,
            20,
            2.0,
            Some(90.0),
            Some(Vec3 {
                x: 0.0,
                y: -0.0,
                z: 7.0,
            }),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }),
            None,
            Some("Instance test".to_string()),
            None,
        );
        eprintln!("Running");

        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        let img = runtime.block_on(async_render(
            Box::new(viewport.clone()),
            &ray_color_bg_color,
            Box::new(scene.to_owned()),
        ));
        write_img_f32(&img, "out/instance_test.png".to_string());
        // let img = viewport.render(&ray_color_bg_color, &scene);
        // write_img_f32(&img, "out/instance_stratified_test.png".to_string());
        let img = runtime.block_on(async_render(
            Box::new(viewport.clone()),
            &ray_color_bg_color,
            Box::new(scene2),
        ));
        // let img = viewport.render(&ray_color_bg_color, &scene2);
        write_img_f32(&img, "out/instance_test2.png".to_string());
    }

    #[test]
    fn volume_test() {
        const PI: f32 = std::f32::consts::PI;
        let samples = 100;
        let _spheres = vec![Sphere::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            0.4,
            Some(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            Some(Material::new_emmiting(
                1.0,
                0.0,
                0.0,
                Vec3 {
                    x: 2.0,
                    y: 0.5,
                    z: 2.0,
                },
            )),
        )];

        let quads = vec![
            //Red
            Quad::new(
                Vec3 {
                    x: -2.0,
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
                METALLIC_M,
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb {
                    0: [0.85, 0.85, 0.85],
                }),
            ),
            //Light
            // Quad::new(
            //     Vec3 {
            //         x: -0.5,
            //         y: -0.5,
            //         z: 1.001,
            //     },
            //     Vec3 {
            //         x: 1.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     Vec3 {
            //         x: 0.0,
            //         y: 1.0,
            //         z: 0.0,
            //     },
            //     Material::new_emmiting(
            //         0.0,
            //         0.0,
            //         1.0,
            //         Vec3 {
            //             x: 4.0,
            //             y: 2.0,
            //             z: 4.0,
            //         },
            //     ),
            //     Vec3 {
            //         x: 0.0,
            //         y: 0.0,
            //         z: 0.0,
            //     },
            //     ImageTexture::from_color(Rgb { 0: [1.0, 1.0, 1.0] }),
            // ),
            //Blue
            Quad::new(
                Vec3 {
                    x: 2.0,
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
                    y: 2.0,
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
                    y: -2.0,
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
            //Green
            Quad::new(
                Vec3 {
                    x: -2.0,
                    y: -2.0,
                    z: 1.0,
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
                Material::new_emmiting(
                    0.0,
                    0.0,
                    1.0,
                    Vec3 {
                        x: 4.0,
                        y: 4.0,
                        z: 4.0,
                    },
                ),
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                ImageTexture::from_color(Rgb { 0: [0.2, 1.0, 0.2] }),
            ),
        ];
        // let scene = Scene::new(spheres, quads.to_owned());
        let mut smoke = Instance::new_box(
            Vec3::new(-1.5, -0.5, -0.5),
            Vec3::new(1.5, 0.5, 0.5),
            ImageTexture::from_color(Vec3::new(0.2, 0.2, 0.2).to_rgb()),
            SCATTER_M,
        );
        smoke.translate(Vec3 {
            x: -0.0,
            y: -1.5,
            z: 2.0,
        });
        smoke.rotate(Vec3 {
            x: 0.0,
            y: PI / 4.0,
            z: 0.0,
        });
        smoke.dist_fn = &const_density;
        smoke.density = 2.0;
        let scene = Scene::new(vec![], quads.to_owned(), vec![smoke.to_owned()]);
        let viewport = Viewport::new_from_res(
            400,
            400,
            samples,
            20,
            2.0,
            Some(90.0),
            Some(Vec3 {
                x: 0.0,
                y: -0.0,
                z: 7.0,
            }),
            Some(Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }),
            None,
            Some("Volume test".to_string()),
            None,
        );
        eprintln!("Running");
        let runtime = tokio::runtime::Builder::new_multi_thread().build().unwrap();
        let img = runtime.block_on(async_render(
            Box::new(viewport.clone()),
            &ray_color_bg_color,
            Box::new(scene),
        ));
        write_img_f32(&img, "out/volume_test.png".to_string());
    }
}
