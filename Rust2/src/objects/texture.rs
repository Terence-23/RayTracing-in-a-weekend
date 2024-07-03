use crate::vec3::vec3::Vec3;
use image::io::Reader as ImageReader;

pub struct ColorResult {
    pub emmited: Vec3,
    pub multiplied: Vec3,
}

pub trait Texture {
    fn color_at(&self, x: f32, y: f32) -> ColorResult;
}

pub struct ConstColorTexture {
    mult: Vec3,
    emmit: Vec3,
}

impl ConstColorTexture {
    pub fn new(mult: Vec3, emmit: Vec3) -> Self {
        Self { mult, emmit }
    }
}

impl Texture for ConstColorTexture {
    fn color_at(&self, _: f32, _: f32) -> ColorResult {
        ColorResult {
            emmited: self.emmit,
            multiplied: self.mult,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImageTexture {
    img: Vec<Vec3>,
    pub width: usize,
    pub height: usize,
    emmit_img: Vec<Vec3>,
    pub emmit_height: usize,
    pub emmit_width: usize,
}

impl ImageTexture {
    pub fn new(
        img: Vec<Vec3>,
        emmit_img: Vec<Vec3>,
        width: usize,
        height: usize,
        emmit_width: usize,
        emmit_height: usize,
    ) -> Self {
        Self {
            width,
            height,
            img,
            emmit_img,
            emmit_height,
            emmit_width,
        }
    }

    pub fn from_path_non_emmisive(path: &str) -> image::ImageResult<Self> {
        let img = ImageReader::open(path)?.decode()?.into_rgb32f();

        let (w, h) = img.dimensions();
        Ok(Self {
            width: w as usize,
            height: h as usize,
            img: img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
            emmit_img: vec![Vec3::ZERO],
            emmit_height: 1,
            emmit_width: 1,
        })
    }
    pub fn from_path_with_emmisive_mask(
        path: &str,
        emmisive_path: &str,
    ) -> image::ImageResult<Self> {
        let img = ImageReader::open(path)?.decode()?.into_rgb32f();
        let e_img = ImageReader::open(emmisive_path)?.decode()?.into_rgb32f();

        let (w, h) = img.dimensions();
        let (ew, eh) = e_img.dimensions();
        Ok(Self {
            width: w as usize,
            height: h as usize,
            img: img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
            emmit_img: e_img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
            emmit_height: eh as usize,
            emmit_width: ew as usize,
        })
    }
}
impl Texture for ImageTexture {
    fn color_at(&self, x: f32, y: f32) -> ColorResult {
        let emmit_x = (x * self.emmit_width as f32).floor() as usize;
        let emmit_y = (y * self.emmit_height as f32).floor() as usize;

        let x = (x * self.width as f32) as usize;
        let y = (y * self.height as f32) as usize;
        return ColorResult {
            emmited: self.emmit_img[emmit_x * self.emmit_width + emmit_y],
            multiplied: self.img[x * self.width + y],
        };
    }
}
