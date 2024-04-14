use crate::vec3::{ray::Ray, vec3::Vec3};

#[derive(PartialEq, Clone, Debug)]
pub struct Hit {
    pub r: Ray,
    pub p: Vec3,
    pub n: Vec3,
    pub t: f32,
}
impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.t.partial_cmp(&other.t)
    }
}
