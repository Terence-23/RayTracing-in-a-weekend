use crate::vec3::vec3::Vec3;

pub trait Rotation {
    fn rotate(&self, v: &Vec3) -> Vec3;
}

#[derive(Debug, Clone, PartialEq)]
pub struct EulerAngles {
    x: f32,
    y: f32,
    z: f32,
}

impl EulerAngles {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
}
impl Rotation for EulerAngles {
    fn rotate(&self, v: &Vec3) -> Vec3 {
        // alfa
        let asin = self.x.sin();
        let acos = self.x.cos();
        //beta
        let bsin = self.y.sin();
        let bcos = self.y.cos();
        //gamma
        let csin = self.z.sin();
        let ccos = self.z.cos();

        Vec3 {
            x: v.x * bcos * ccos
                + v.y * (asin * bsin * ccos - asin * ccos)
                + v.z * (acos * bsin * ccos + asin * csin),
            y: v.x * bcos * csin
                + v.y * (asin * bsin * csin + acos * ccos)
                + v.z * (acos * bsin * csin - asin * ccos),
            z: v.x * -bsin + v.y * asin * bcos + v.z * acos * bcos,
        }
    }
}
