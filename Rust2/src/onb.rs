use crate::vec3::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ONB {
    pub fn new() -> Self {
        Self {
            u: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            v: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            w: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        }
    }
    pub fn new_from_w(w: Vec3) -> Self {
        let unit_w = w.unit();
        let a = if unit_w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::cross(&unit_w, a).unit();
        let u = Vec3::cross(&unit_w, v).unit();
        Self { u, v, w: unit_w }
    }

    pub fn from_local(&self, v: Vec3) -> Vec3 {
        self.u * v.x + self.v * v.y + self.w * v.z
    }
    pub fn from_global(&self, a: Vec3) -> Vec3 {
        Vec3 {
            x: a.dot(self.u),
            y: a.dot(self.v),
            z: a.dot(self.w),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::vec3::vec3::Vec3;

    use super::ONB;
    #[test]
    fn random100_from_local() {
        for _ in 0..100 {
            let onb = ONB::new_from_w(Vec3::random_unit_vec());
            let vec = Vec3::random_unit_vec();
            eprintln!("{}", (vec.length() - onb.from_local(vec).length()).abs());
            debug_assert!((vec.length() - onb.from_local(vec).length()).abs() < 1e-6);
        }
    }
    #[test]
    fn random100() {
        for _ in 0..100 {
            let onb = ONB::new_from_w(Vec3::random_unit_vec());
            let vec = Vec3::random_unit_vec();
            eprintln!(
                "{:?}",
                (vec - onb.from_local(onb.from_global(vec))).length()
            );
            debug_assert!((vec - onb.from_local(onb.from_global(vec))).length() < 4e-7);
        }
    }
}
