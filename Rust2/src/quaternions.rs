use std::{
    f32::consts::PI,
    ops::{Add, Mul, MulAssign},
};

use crate::{
    rotation::{EulerAngles, Rotation},
    vec3::vec3::Vec3,
};

pub const ZERO_ROTATION: Quaternion = Quaternion {
    w: 1.0,
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for &Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Self) -> Self::Output {
        Quaternion {
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Mul<f32> for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion {
            w: self.w * rhs,
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for &Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        self.hamilton(rhs)
    }
}
impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.hamilton(&rhs);
    }
}
impl From<EulerAngles> for Quaternion {
    fn from(v: EulerAngles) -> Self {
        (&v).into()
    }
}
impl From<&EulerAngles> for Quaternion {
    fn from(v: &EulerAngles) -> Self {
        let cx = v.x.cos();
        let cy = v.y.cos();
        let cz = v.z.cos();

        let sx = v.x.sin();
        let sy = v.y.sin();
        let sz = v.z.sin();

        Quaternion {
            w: cx * cy * cz + sx * sy * sz,
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz - sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
        }
    }
}

impl From<&Quaternion> for EulerAngles {
    fn from(q: &Quaternion) -> Self {
        // roll (x-axis rotation)
        let sinr_cosp = 2.0 * (q.w * q.x + q.y * q.z);
        let cosr_cosp = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);
        // pitch (y-axis rotation)
        let sinp = (1.0 + 2.0 * (q.w * q.y - q.x * q.z)).sqrt();
        let cosp = (1.0 - 2.0 * (q.w * q.y - q.x * q.z)).sqrt();

        // yaw (z-axis rotation)
        let siny_cosp = 2.0 * (q.w * q.z + q.x * q.y);
        let cosy_cosp = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);

        return EulerAngles {
            x: f32::atan2(sinr_cosp, cosr_cosp),
            y: 2.0 * f32::atan2(sinp, cosp) - PI * 0.5,
            z: f32::atan2(siny_cosp, cosy_cosp),
        };
    }
}
impl From<Quaternion> for EulerAngles {
    fn from(value: Quaternion) -> Self {
        EulerAngles::from(&value)
    }
}

impl Quaternion {
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Quaternion {
            w,
            x: x,
            y: y,
            z: z,
        }
    }
    pub fn len2(&self) -> f32 {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f32 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn dot(&self, oth: &Quaternion) -> f32 {
        self.w * oth.w + self.x * oth.x + self.y * oth.y + self.z * oth.z
    }
    pub fn hamilton(&self, oth: &Quaternion) -> Quaternion {
        Quaternion {
            w: self.w * oth.w - self.x * oth.x - self.y * oth.y - self.z * oth.z,
            x: self.w * oth.x + self.x * oth.w + self.y * oth.z - oth.y * self.z,
            y: self.w * oth.y - self.x * oth.z + self.y * oth.w + self.z * oth.x,
            z: self.w * oth.z + self.x * oth.y - self.y * oth.x + self.w * oth.z,
        }
    }
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn get_vec(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl From<&Vec3> for Quaternion {
    fn from(v: &Vec3) -> Self {
        Quaternion {
            w: 0.0,
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Rotation for Quaternion {
    fn rotate(&self, v: &Vec3) -> Vec3 {
        let q = self * (1.0 / self.len());
        let cq = q.conjugate();
        let rotated: Quaternion = v.into();

        q.hamilton(&rotated).hamilton(&cq).get_vec()
    }

    fn add(&self, r: impl Rotation) -> Self {
        self.hamilton(&r.into())
    }
}
