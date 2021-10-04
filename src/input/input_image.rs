use super::InputFile;
use image::{GenericImage, GenericImageView, ImageBuffer, Luma, RgbImage};
use std::path::Path;
use std::{ops::Index, process::Output};

pub struct InputImage<const H: usize, const W: usize>
where
    [(); H * W]: Sized,
{
    index: usize,
    height: usize,
    width: usize,
    data: [u8; H * W],
    label: Option<u8>,
}

impl<const H: usize, const W: usize> InputImage<H, W>
where
    [(); H * W]: Sized,
{
    const SAVE_PATH: &'static str = "data/images/";
    pub fn new(index: usize, image_data: &[u8; H * W]) -> InputImage<H, W> {
        InputImage {
            index,
            label: Option::None,
            height: H,
            width: W,
            data: image_data.clone(),
        }
    }

    pub fn to_file(&self) {
        self.to_image_buffer()
            .save(format!("{}img_{}.png", Self::SAVE_PATH, self.index));
    }

    pub fn to_image_buffer(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let mut img = ImageBuffer::from_fn(self.height as u32, self.width as u32, |x, y| {
            image::Luma([self[(x as usize + (y as usize * W))]])
        });
        img
    }
}

impl<const H: usize, const W: usize> Index<usize> for InputImage<H, W>
where
    [(); H * W]: Sized,
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn load_input_file() -> InputFile<'static> {
        InputFile::new("data/t10k-images-idx3-ubyte")
    }

    #[test]
    fn new_input_image_test() {
        let i = load_input_file();
        let img = InputImage::<28, 28>::new(3, &i.data_for_image(3));
        let offset = 2688 - ((784 * 3) + 16) + 0x1B;
        assert_eq!(img[offset], 188)
    }

    #[test]
    fn input_image_to_file_test() {
        let image_index = 89;
        let file_path = format!("{}img_{}.png", InputImage::<28, 28>::SAVE_PATH, image_index);
        let i = load_input_file();
        let img = InputImage::<28, 28>::new(image_index, &i.data_for_image(image_index));

        std::fs::remove_file(&file_path);
        assert_eq!(Path::new(&file_path).exists(), false);
        img.to_file();
        assert_eq!(Path::new(&file_path).exists(), true);
        std::fs::remove_file(&file_path);
    }
}
