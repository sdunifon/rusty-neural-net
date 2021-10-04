use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

pub fn main_image() {
    let img: RgbImage = ImageBuffer::new(512, 512);
    // Construct a new by repeated calls to the supplied closure.
    let mut img = ImageBuffer::from_fn(512, 512, |x, y| {
        if x % 2 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    let pixel = img[(100, 100)];

    dbg!("pixel {}", pixel);
}
// Construct a new RGB ImageBuffer with the specified width and height.
