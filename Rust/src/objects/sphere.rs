use super::{
    materials::{Material, EMPTY_M},
    Hit, Object,
};
use crate::{
    objects::NO_HIT,
    texture::texture::{ImageTexture, Texture},
    vec3::{ray::Ray, vec3::Vec3},
};
use core::fmt;
use json::JsonValue;

#[derive(Clone)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub col_mod: Vec3,
    pub mat: Material,
    pub velocity: Vec3,
    pub texture: ImageTexture,
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin
            && self.radius == other.radius
            && self.col_mod == other.col_mod
            && self.mat == other.mat
            && self.velocity == other.velocity
    }
}

impl fmt::Debug for Sphere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sphere")
            .field("origin", &self.origin)
            .field("radius", &self.radius)
            .field("col_mod", &self.col_mod)
            .field("mat", &self.mat)
            .field("velocity", &self.velocity)
            .finish()
    }
}

impl Into<JsonValue> for Sphere {
    fn into(self) -> JsonValue {
        json::object! {
            origin: self.origin,
            radius: self.radius,
            col_mod: self.col_mod,
            material: self.mat,
            velocity: self.velocity,
            texture: self.texture,
        }
    }
}
impl TryFrom<JsonValue> for Sphere {
    type Error = crate::viewport::errors::ParseError;

    fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
        Ok(Sphere {
            origin: match Vec3::try_from(value["origin"].to_owned()) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            radius: match value["radius"].as_f32() {
                Some(x) => x,
                None => return Err(Self::Error { source: None }),
            },
            col_mod: match Vec3::try_from(value["col_mod"].to_owned()) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            mat: match Material::try_from(value["material"].to_owned()) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            velocity: match Vec3::try_from(value["velocity"].to_owned()) {
                Ok(x) => x,
                Err(e) => return Err(e),
            },
            texture: match ImageTexture::try_from(value["texture"].to_owned()) {
                Ok(t) => t,
                Err(e) => {
                    println!("No texture");
                    return Err(e);
                }
            },
        })
    }
}
impl Object for Sphere {
    fn collide(&self, r: Ray) -> bool {
        let oc = r.origin - self.origin;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        return b * b - (4.0 * a * c) > 0.0;
    }
    fn collision_normal(&self, r: Ray, mint: f32, maxt: f32) -> Hit {
        let origin = self.origin + self.velocity * r.time;
        let oc = r.origin - origin;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let d = b * b - a * c;

        if d < 0.0 {
            return NO_HIT;
        }

        let mut x = (-b - d.sqrt()) / a;
        if x < mint {
            // dbg!(r.direction.length());
            // dbg!(r.at(x));
            // dbg!(mint);
            // dbg!(x);
            // dbg!((r.at(x) - origin).length());
            x = (-b + d.sqrt()) / a
        }

        if x < mint || x > maxt {
            // dbg!(x);
            return NO_HIT;
        }
        // println!("mint: {}", mint);
        debug_assert_ne!(x, 0.0);
        let normal = (r.at(x) - origin).unit();

        let u: f32 = (f32::atan2(-normal.z, normal.x) + std::f32::consts::PI)
            * std::f32::consts::FRAC_1_PI
            * 0.5;
        let v: f32 = 1.0 - (std::f32::consts::FRAC_1_PI * f32::acos(-normal.y));

        debug_assert!(u <= 1.0 && v >= 0.0, "U too big");
        debug_assert!(v <= 1.0 && v >= 0.0, "V too big");

        let tex_x = (u * (self.texture.row - 1) as f32).floor() as usize;
        let tex_y = (v * (self.texture.col - 1) as f32).floor() as usize;

        Hit {
            t: x,
            normal: normal,
            point: r.at(x),
            mat: self.mat,
            col_mod: self.texture.color_at(tex_x, tex_y, r.at(x)) * self.col_mod,
        }
    }
}
#[allow(dead_code)]
impl Sphere {
    pub fn new(origin: Vec3, r: f32, col_mod: Option<Vec3>, mat: Option<Material>) -> Sphere {
        Sphere {
            origin: origin,
            radius: r,
            col_mod: match col_mod {
                Some(n) => n,
                None => Vec3::new(1.0, 1.0, 1.0),
            },
            mat: match mat {
                Some(n) => n,
                None => EMPTY_M,
            },
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            texture: ImageTexture::from_color(match col_mod {
                Some(n) => n.to_rgb(),
                None => Vec3::new(1.0, 1.0, 1.0).to_rgb(),
            }),
        }
    }
    pub fn new_moving(
        origin: Vec3,
        r: f32,
        col_mod: Option<Vec3>,
        mat: Option<Material>,
        velocity: Vec3,
    ) -> Sphere {
        Sphere {
            origin: origin,
            radius: r,
            col_mod: match col_mod {
                Some(n) => n,
                None => Vec3::new(1.0, 1.0, 1.0),
            },
            mat: match mat {
                Some(n) => n,
                None => EMPTY_M,
            },
            velocity,
            texture: ImageTexture::from_color(match col_mod {
                Some(n) => n.to_rgb(),
                None => Vec3::new(1.0, 1.0, 1.0).to_rgb(),
            }),
        }
    }
    pub fn new_with_texture(
        origin: Vec3,
        r: f32,
        col_mod: Option<Vec3>,
        mat: Option<Material>,
        tex: ImageTexture,
    ) -> Sphere {
        Sphere {
            origin: origin,
            radius: r,
            col_mod: match col_mod {
                Some(n) => n,
                None => Vec3::new(1.0, 1.0, 1.0),
            },
            mat: match mat {
                Some(n) => n,
                None => EMPTY_M,
            },
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            texture: tex,
        }
    }
    pub fn new_moving_with_texture(
        origin: Vec3,
        r: f32,
        col_mod: Option<Vec3>,
        mat: Option<Material>,
        velocity: Vec3,
        tex: ImageTexture,
    ) -> Sphere {
        Sphere {
            origin: origin,
            radius: r,
            col_mod: match col_mod {
                Some(n) => n,
                None => Vec3::new(1.0, 1.0, 1.0),
            },
            mat: match mat {
                Some(n) => n,
                None => EMPTY_M,
            },
            velocity: velocity,
            texture: tex,
        }
    }
}
