use super::InputImage;
use rayon::prelude::*;
use std::fs::File;

use std::os::unix::fs::FileExt;

pub struct InputFile<'a> {
    file_name: &'a str,
    file: File,
    image_height: u8,
    image_width: u8,
    count: u16,
}

impl<'a> InputFile<'a> {
    const HEADER_SIZE: usize = 16;
    const HEADER_HEIGHT_OFFSET: usize = 11;
    const HEADER_WIDTH_OFFSET: usize = 15;
    const IMAGE_BUFFER_SIZE: usize = 28 * 28; //using so we don't have to allocate on stack.. protectino in image_buffer_size method

    pub fn new(file_name: &'a str) -> Self {
        let file = File::open("data/t10k-images-idx3-ubyte").unwrap();
        let (count, image_height, image_width) = Self::get_image_count_height_and_width(&file);
        let input_file = Self {
            file_name,
            file,
            image_height,
            image_width,
            count,
        };
        input_file.image_buffer_size();
        input_file
    }
    pub fn get_image_count_height_and_width(input_buffer: &dyn FileExt) -> (u16, u8, u8) {
        let mut buf: [u8; 16] = [0; Self::HEADER_SIZE];
        input_buffer.read_exact_at(&mut buf, 0);
        let height = buf[Self::HEADER_HEIGHT_OFFSET];
        let width = buf[Self::HEADER_WIDTH_OFFSET];
        let count: u16 = (buf[6] as u16 * 256) + buf[7] as u16;

        (count.into(), height.into(), width.into())
    }

    pub fn image_buffer_size(&self) -> usize {
        if (self.image_width as usize * self.image_height as usize) != Self::IMAGE_BUFFER_SIZE {
            panic!("ERROR: incorrect constant. Size different that IMAGE_BUFFER_SIZE constant h{} * w{} = {} != IMAGE_BUFFER_SIZE:{} ", self.image_height, self.image_width, self.image_height as usize * self.image_width as usize, Self::IMAGE_BUFFER_SIZE);
        }
        Self::IMAGE_BUFFER_SIZE
    }

    pub fn data_for_image(&self, image_index: usize) -> [u8; InputFile::IMAGE_BUFFER_SIZE] {
        let mut buf = [0u8; Self::IMAGE_BUFFER_SIZE];

        let byte_offset = Self::HEADER_SIZE + (Self::IMAGE_BUFFER_SIZE * image_index);
        self.file
            .read_exact_at(&mut buf, byte_offset.try_into().unwrap());
        buf
    }

    pub fn image(&self, index: usize) -> InputImage<28, 28> {
        InputImage::new(index, &self.data_for_image(index))
    }

    pub fn render_image_files(&self) {
        for n in 0..self.count {
            self.image(n.into()).to_file();
        }
    }
    pub fn render_image_files_parallel(&self) {
        (0..self.count)
            .into_par_iter()
            .for_each(|x| self.image(x.into()).to_file());
    }
}

//#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    fn load_input_file() -> InputFile<'static> {
        InputFile::new("data/t10k-images-idx3-ubyte")
    }
    #[test]
    fn test_file_height_width() {
        assert_eq!(
            InputFile::get_image_count_height_and_width(
                &File::open("data/t10k-images-idx3-ubyte").unwrap()
            ),
            (10000, 28, 28)
        );
    }
    #[test]
    fn test_input_file_height_width() {
        let i = load_input_file();
        assert_eq!((i.count, i.image_height, i.image_width), (10000, 28, 28));
    }
    #[test]
    fn test_image_buffer_size() {
        let i = load_input_file();
        assert_eq!(i.image_buffer_size(), 28 * 28);
    }
    #[test]
    fn test_data_for_image() {
        let i = load_input_file();
        dbg!("a:{}", &i.data_for_image(0)[206..300],);
        let mut image_index = 0;
        assert_eq!(
            i.data_for_image(image_index)[196 - ((784 * image_index) + 16) + 0x1b],
            36
        );
        image_index = 2;
        assert_eq!(
            i.data_for_image(image_index)[1904 - ((784 * image_index) + 16) + 0x03],
            254
        );
    }
    #[test]
    fn test_image() {
        let image_index = 4;
        let i = load_input_file();
        assert_eq!(
            i.image(image_index)[3444 - ((784 * (image_index)) + 16) + 0x18],
            242
        );
    }
    #[bench]
    fn render_image_files_benchmark(b: &mut test::Bencher) {
        let f = InputFile::new("data/t10k-images-idx3-ubyte");
        b.iter(|| f.render_image_files())
    }
}
