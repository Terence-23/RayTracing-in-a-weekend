use crate::onb::ONB;
use crate::vec3::vec3::Vec3;

#[derive(Clone)]
pub struct Camera {
    v_up: Vec3,
    pub left_top: Vec3,
    pub origin: Vec3,
    onb: ONB,
    width: f32,
    height: f32,
    aspect_ratio: f32, // width/height
    pub delta_x: Vec3,
    pub delta_y: Vec3,
    pub(super) lens_radius: f32,
}

impl Camera {
    pub fn new(
        aspect: f32,
        origin: Vec3,
        vup: Vec3,
        dir: Vec3,
        vfov: f32,
        lens_radius: f32,
    ) -> Self {
        let w = -dir;
        let u = vup.cross(w).unit();
        let v = w.cross(u);
        let onb = ONB { u, v, w };
        let h = (vfov * std::f32::consts::PI / 360.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect * viewport_height;

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_x = viewport_u;
        let pixel_delta_y = viewport_v;

        Self {
            v_up: vup,
            left_top: -w - viewport_u / 2.0 - viewport_v / 2.0,
            origin: origin,
            onb: onb,
            width: viewport_width,
            height: viewport_height,
            aspect_ratio: aspect,
            delta_x: pixel_delta_x,
            delta_y: pixel_delta_y,
            lens_radius,
        }
    }
}
