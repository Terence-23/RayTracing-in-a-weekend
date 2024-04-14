#[allow(dead_code)]
pub mod img_writer {

    use image::{ImageBuffer, Rgb};

    pub fn write_img_f32(arr: &Vec<Vec<Rgb<f32>>>, filename: String) {
        let mut img = ImageBuffer::new(arr[0].len() as u32, arr.len() as u32);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            let rgb = arr[y as usize][x as usize];
            *pix = Rgb([
                (rgb.0[0] * 255.0).clamp(0.0, 255.0).round() as u8,
                (rgb.0[1] * 255.0).clamp(0.0, 255.0).round() as u8,
                (rgb.0[2] * 255.0).clamp(0.0, 255.0).round() as u8,
            ]);
        }

        img.save(filename).unwrap()
    }

    pub fn write_img(arr: Vec<Vec<Rgb<u8>>>, filename: String) {
        let mut img = ImageBuffer::new(arr[0].len() as u32, arr.len() as u32);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            *pix = arr[y as usize][x as usize];
        }

        img.save(filename).unwrap()
    }
    #[allow(unused_imports)]
    mod tests {

        use super::*;
        #[test]
        pub fn test_write() {
            let width = 256;
            let height = 256;

            let mut arr = vec![vec![Rgb([0 as u8, 0 as u8, 0 as u8]); width]; height];

            for i in 0..width {
                for j in 0..height {
                    let rgb = Rgb([
                        (i as f32) / (width - 1) as f32,
                        (j as f32) / (height - 1) as f32,
                        0.25,
                    ]);
                    arr[j][i] = Rgb([
                        (rgb.0[0] * 255.0).round() as u8,
                        (rgb.0[1] * 255.0).round() as u8,
                        (rgb.0[2] * 255.0).round() as u8,
                    ]);
                }
            }

            write_img(arr, "test_out/test.png".to_string())
        }
    }
}
