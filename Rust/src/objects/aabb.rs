use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Axis {
    X,
    Y,
    Z,
}
impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=2) {
            // rand 0.8
            0 => Axis::X,
            1 => Axis::Y,
            _ => Axis::Z,
        }
    }
}

mod aabb;
pub type AABB = aabb::AABB;

mod qaabb;
pub type QuadAABB = qaabb::QuadAABB;

mod iaabb;
pub type IAABB = iaabb::IAABB;
