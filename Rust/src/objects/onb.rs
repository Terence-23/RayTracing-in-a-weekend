use crate::vec3::vec3::Vec3;

struct ONB {
    u: Vec3,
    v: Vec3,
    w: Vec3,
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
        let a = if (unit_w.x.abs() > 0.9) {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = Vec3::cross(&unit_w, a).unit();
        let u = Vec3::cross(&unit_w, v);
        Self { u, v, w: unit_w }
    }

    pub fn local(self, v: Vec3) -> Vec3 {
        self.u * v.x + self.v * v.x + self.w * v.z
    }
}
