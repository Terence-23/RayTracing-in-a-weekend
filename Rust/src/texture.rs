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
        pub noise_scale: f32,
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
                noise: None,
                noise_scale: 1.0,
            })
        }
    }

    const PERLIN_POINT_COUNT: usize = 256;
    #[derive(Clone, Debug)]
    pub struct PerlinNoise {
        ranfloat: [f32; PERLIN_POINT_COUNT],
        ranvec: [Vec3; PERLIN_POINT_COUNT],
        perm_x: [usize; PERLIN_POINT_COUNT],
        perm_y: [usize; PERLIN_POINT_COUNT],
        perm_z: [usize; PERLIN_POINT_COUNT],
    }

    impl PerlinNoise {
        fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
            let mut accum = 0.0;
            for i in 0..2 {
                for j in 0..2usize {
                    for k in 0..2usize {
                        accum += (u * i as f32 + (1.0 - u) * (1 - i) as f32)
                            * (j as f32 * v + (1.0 - j as f32) * (1.0 - v))
                            * (k as f32 * w + (1.0 - k as f32) * (1.0 - w))
                            * c[i][j][k];
                    }
                }
            }
            return accum;
        }
        fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
            let uu = u * u * (3.0 - 2.0 * u);
            let vv = v * v * (3.0 - 2.0 * v);
            let ww = w * w * (3.0 - 2.0 * w);
            let mut accum = 0.0;

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f32;
                        let j = j as f32;
                        let k = k as f32;
                        let weight_v = Vec3::new(u - i, v - j, w - k);
                        accum += (i * uu + (1.0 - i) * (1.0 - uu))
                            * (j * vv + (1.0 - j) * (1.0 - vv))
                            * (k * ww + (1.0 - k) * (1.0 - ww))
                            * c[i as usize][j as usize][k as usize].dot(weight_v);
                    }
                }
            }

            return accum;
        }

        pub fn new() -> Self {
            let mut rng = thread_rng();
            let mut ran_float = [0.0; PERLIN_POINT_COUNT];
            let mut ranvec = [Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }; PERLIN_POINT_COUNT];
            let between = Uniform::new(0.0f32, 1.0);

            for i in 0..PERLIN_POINT_COUNT {
                ranvec[i] = Vec3::random(-1.0, 1.0).unit();
            }

            for i in 0..PERLIN_POINT_COUNT {
                ran_float[i] = between.sample(&mut rng);
            }

            return Self {
                ranfloat: ran_float,
                ranvec,
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
        pub fn value(&self, p: Vec3) -> f32 {
            (1.0 + self.noise(p)) * 0.5
        }

        pub fn noise(&self, p: Vec3) -> f32 {
            let u = p.x - p.x.floor();
            let v = p.y - p.y.floor();
            let w = p.z - p.z.floor();

            let i = p.x.floor() as isize;
            let j = p.y.floor() as isize;
            let k = p.z.floor() as isize;
            let mut c = [[[Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }; 2]; 2]; 2];

            for di in 0..2isize {
                for dj in 0..2isize {
                    for dk in 0..2isize {
                        c[di as usize][dj as usize][dk as usize] = self.ranvec[self.perm_x
                            [((i + di) & 255) as usize]
                            ^ self.perm_y[((j + dj) & 255) as usize]
                            ^ self.perm_z[((k + dk) & 255) as usize]];
                    }
                }
            }
            return Self::perlin_interp(c, u, v, w);
        }

        pub fn turb(&self, p: Vec3, depth: usize) -> f32 {
            let mut accum: f32 = 0.0;
            let mut temp_p = p;
            let mut weight = 1.0;

            for _ in 0..depth {
                accum += weight * self.noise(temp_p);
                weight *= 0.5;
                temp_p *= 2.0;
            }

            return accum.abs();
        }
    }

    impl ImageTexture {
        pub fn new(img: Vec<Vec3>, width: usize, height: usize) -> Self {
            Self {
                row: width,
                col: height,
                img,
                noise: None,
                noise_scale: 1.0,
            }
        }
        pub fn from_color(color: Rgb<f32>) -> Self {
            Self {
                img: vec![Vec3::from_rgb(color)],
                col: 1,
                row: 1,
                noise: None,
                noise_scale: 1.0,
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
                noise_scale: 1.0,
            })
        }

        pub fn new_with_noise(img: Vec<Vec3>, width: usize, height: usize, scale: f32) -> Self {
            Self {
                row: width,
                col: height,
                img,
                noise: Some(PerlinNoise::new()),
                noise_scale: scale,
            }
        }
        pub fn from_color_noise(color: Rgb<f32>, scale: f32) -> Self {
            Self {
                img: vec![Vec3::from_rgb(color)],
                col: 1,
                row: 1,
                noise: Some(PerlinNoise::new()),
                noise_scale: scale,
            }
        }
        pub fn from_path_noise(path: &str, scale: f32) -> image::ImageResult<Self> {
            let img = ImageReader::open(path)?.decode()?.into_rgb32f();

            let (w, h) = img.dimensions();
            Ok(Self {
                row: w as usize,
                col: h as usize,
                img: img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
                noise: Some(PerlinNoise::new()),
                noise_scale: scale,
            })
        }
    }
    impl Texture for ImageTexture {
        fn color_at(&self, x: usize, y: usize, p: Vec3) -> Vec3 {
            let noise_mult = match &self.noise {
                Some(n) => n.noise(p / self.noise_scale),
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
