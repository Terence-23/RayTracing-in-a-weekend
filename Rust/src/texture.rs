
#[allow(dead_code)]
pub mod texture{
    
    use image::io::Reader as ImageReader;

    use image::Rgb;
    use json::JsonValue;

    use crate::{vec3::vec3::Vec3, viewport::errors};

    pub trait Texture{
        fn color_at(&self, x: usize, y:usize) -> Vec3;
    }
    
    #[derive(Clone, Debug)]
    pub struct ImageTexture{
        img: Vec<Vec3>,
        pub row: usize,
        pub col: usize,
    }
    impl TryFrom<JsonValue> for ImageTexture{
        type Error = errors::ParseError;

        fn try_from(value: JsonValue) -> Result<Self, Self::Error> {
            let mut img = Vec::new();
            match value["img"].to_owned()
            {
                JsonValue::Array(v) => for col in v{
                    let pval = Vec3::try_from(col)?;
                    img.push(pval);
                }
                _ => return Err(Self::Error{source: None})
            }
            let row = match value["row"].as_usize(){
                Some(n) => n,
                None => return Err(Self::Error{source: None})
            };
            let col = match value["col"].as_usize(){
                Some(n) => n,
                None => return Err(Self::Error{source: None})
            };

            Ok(Self{row: row, col: col, img: img})
        }
    }

impl ImageTexture {
    pub fn new(img: Vec<Vec3>, width: usize, height: usize) -> Self { Self {row: width, col:height, img:img } }
    pub fn from_color(color: Rgb<f32>) -> Self { 
        Self {
            img : vec![Vec3::from_rgb(color)],
            col: 1, row: 1
        }
    }
    pub fn from_path(path : &str) -> image::ImageResult<Self>{
        let img =  ImageReader::open(path)?.decode()?.into_rgb32f();
        
        let (w, h) = img.dimensions();
        Ok(
            Self{
                row: w as usize,
                col: h as usize,
                img: img.pixels().map(|col| Vec3::from_rgb_ref(col)).collect(),
            }
        )
    }
} 
impl Texture for ImageTexture {
        fn color_at(&self, x: usize, y:usize) -> Vec3 {

            self.img[y * self.row + x]
        }
    }
impl Into<JsonValue> for ImageTexture{
    fn into(self) -> JsonValue {
        json::object! {
            row: self.row,
            col: self.col,
            img: self.img
        }
    }
}
}