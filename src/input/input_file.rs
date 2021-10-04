use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::os::unix::fs::FileExt;

pub fn main() -> io::Result<()> {
    println!("in file main");
    let mut file = File::open("data/t10k-images-idx3-ubyte")?;
    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = file.read(&mut buffer[..])?;

    println!("The bytes: {:?}", &buffer[..n]);
    Ok(())
}

struct InputFile<'a> {
    file_name: &'a str,
    file: File,
    image_height: u8,
    image_width: u8,
    count: u16,
}

impl<'a> InputFile<'a> {
    pub fn new(file_name: &'a str) -> Self {
        let mut file = File::open("data/t10k-images-idx3-ubyte").unwrap();
        let (count, image_height, image_width) = Self::get_image_count_height_and_width(&file);
        Self {
            file_name,
            file,
            image_height,
            image_width,
            count,
        }
    }
    pub fn get_image_count_height_and_width(input_buffer: &dyn FileExt) -> (u16, u8, u8) {
        let mut buf: [u8; 16] = [0; 16];
        input_buffer.read_exact_at(&mut buf, 0);
        let height = buf[11];
        let width = buf[15];
        let count: u16 = (buf[6] as u16 * 256) + buf[7] as u16;

        (count.into(), height.into(), width.into())
    }

    pub fn image_buffer_size(&self) -> usize {
        (self.image_width as usize * self.image_height as usize).into()
    }

    pub fn data_for_image(image_index: usize) {}
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
