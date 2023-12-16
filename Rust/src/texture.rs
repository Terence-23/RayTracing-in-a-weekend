#[allow(dead_code)]
pub mod texture {

    use image::io::Reader as ImageReader;

    use image::Rgb;
    use json::JsonValue;
    use rand::{
        distributions::{Distribution, Uniform},
        rngs::ThreadRng,
        thread_rng, Rng,
    };

    use crate::{vec3::vec3::Vec3, viewport::errors};

    pub trait Texture {
        fn color_at(&self, x: usize, y: usize, p: Vec3) -> Vec3;
    }

    #[derive(Clone, Debug)]
    pub struct ImageTexture {
        img: Vec<Vec3>,
        pub row: usize,
        pub col: usize,
        pub noise: Option<PerlinNoise>,
    }
    impl TryFrom<JsonValue> for ImageTexture {
        type Error = errors::ParseError;

        fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
            let mut img = Vec::new();
            match value["img"].to_owned() {
                JsonValue::Array(v) => {
                    for col in v {
                        let pval = Vec3::try_from(col)?;
                        img.push(pval);
                    }
                }
                _ => return Err(Self::Error { source: None }),
            }
            let row = match value["row"].as_usize() {
                Some(n) => n,
                None => return Err(Self::Error { source: None }),
            };
            let col = match value["col"].as_usize() {
                Some(n) => n,
                None => return Err(Self::Error { source: None }),
            };

            Ok(Self {
                row: row,
                col: col,
                img: img,
                noise: None
            })
        }
    }

    const PERLIN_POINT_COUNT: usize = 256;
    #[derive(Clone, Debug)]
    pub struct PerlinNoise {
        ranfloat: [f32; PERLIN_POINT_COUNT],
        perm_x: [usize; PERLIN_POINT_COUNT],
        perm_y: [usize; PERLIN_POINT_COUNT],
        perm_z: [usize; PERLIN_POINT_COUNT],
    }

    impl PerlinNoise {
        pub fn new() -> Self {
            let mut rng = thread_rng();
            let mut ran_float = [0.0; PERLIN_POINT_COUNT];
            let between = Uniform::new(0.0f32, 1.0);

            for i in 0..PERLIN_POINT_COUNT {
                ran_float[i] = between.sample(&mut rng);
            }

            return Self {
                ranfloat: ran_float,
                perm_x: Self::create_permute(&mut rng),
                perm_y: Self::create_permute(&mut rng),
                perm_z: Self::create_permute(&mut rng),
            };
        }

        fn create_permute(rng: &mut ThreadRng) -> [usize; PERLIN_POINT_COUNT] {
            let mut p: [usize; PERLIN_POINT_COUNT] = (0..PERLIN_POINT_COUNT)
                .collect::<Vec<usize>>()
                .try_into()
                .expect("texture.rs line 76; range 0..256 to array of [usize;256] failed");
            for i in PERLIN_POINT_COUNT - 1..0 {
                let target = (*rng).gen_range(0..i);
                let tmp = p[i];
                p[i] = p[target];
                p[target] = tmp;
            }
            p
        }

        fn noise(&self, p: Vec3) -> f32 {
            let i = ((4.0 * p.x) as usize) & 255;
            let j = ((4.0 * p.y) as usize) & 255;
            let k = ((4.0 * p.z) as usize) & 255;

            return self.ranfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]];
        }
    }

    impl ImageTexture {
        pub fn new(img: Vec<Vec3>, width: usize, height: usize) -> Self {
            Self {
                row: width,
                col: height,
                img,
                noise: None,
            }
        }
        pub fn from_color(color: Rgb<f32>) -> Self {
            Self {
                img: vec![Vec3::from_rgb(color)],
                col: 1,
                row: 1,
                noise: None,
            }
        }
        pub fn from_path(path: &str) -> image::ImageResult<Self> {
            let img = ImageReader::open(path)?.decode()?.into_rgb32f();

            let (w, h) = img.dimensions();
            Ok(Self {
                row: w as usize,
                col: h as usize,
                img: img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
                noise: None,
            })
        }
        
        pub fn new_with_noise(img: Vec<Vec3>, width: usize, height: usize) -> Self {
            Self {
                row: width,
                col: height,
                img,
                noise: Some(PerlinNoise::new()),
            }
        }
        pub fn from_color_noise(color: Rgb<f32>) -> Self {
            Self {
                img: vec![Vec3::from_rgb(color)],
                col: 1,
                row: 1,
                noise: Some(PerlinNoise::new()),
            }
        }
        pub fn from_path_noise(path: &str) -> image::ImageResult<Self> {
            let img = ImageReader::open(path)?.decode()?.into_rgb32f();

            let (w, h) = img.dimensions();
            Ok(Self {
                row: w as usize,
                col: h as usize,
                img: img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
                noise: Some(PerlinNoise::new()),
            })
        }
        
    }
    impl Texture for ImageTexture {
        fn color_at(&self, x: usize, y: usize, p: Vec3) -> Vec3 {
            let noise_mult = match &self.noise{
                Some(n) => n.noise(p),
                None => 1.0,
            };
            self.img[y * self.row + x] * noise_mult
        }
    }
    impl Into<JsonValue> for ImageTexture {
        fn into(self) -> JsonValue {
            json::object! {
                row: self.row,
                col: self.col,
                img: self.img
            }
        }
    }
}
