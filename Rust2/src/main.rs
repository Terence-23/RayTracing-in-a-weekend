pub mod objects;
pub mod onb;
#[allow(unused)]
pub mod postprocessing;
pub mod quaternions;
pub mod rotation;
pub mod vec3;
pub mod viewport;
pub mod write_img;

#[cfg(test)]
#[allow(unused)]
pub mod tests;

use image::ImageResult;

fn main() -> ImageResult<()> {
    println!("Hello, world!");
    Ok(())
}
