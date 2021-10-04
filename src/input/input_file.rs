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
    image_height: usize,
    image_width: usize,
}

impl<'a> InputFile<'a> {
    pub fn new(file_name: &'a str) -> Self {
        let mut file = File::open("data/t10k-images-idx3-ubyte").unwrap();
        let (height, width) = Self::get_image_height_and_width(&file);
        Self {
            file_name,
            file,
            image_height: height,
            image_width: width,
        }
    }

    pub fn get_image_height_and_width(input_buffer: &dyn FileExt) -> (usize, usize) {
        let mut buf: [u8; 5] = [0; 5];
        input_buffer.read_exact_at(&mut buf, 11);
        let height = buf[0];
        let width = buf[4];
        (height.into(), width.into())
    }

    pub fn data_for_image(image_num: usize) {}
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
            InputFile::get_image_height_and_width(
                &File::open("data/t10k-images-idx3-ubyte").unwrap()
            ),
            (28, 28)
        );
    }
    #[test]
    fn test_input_file_height_width() {
        let i = load_input_file();
        assert_eq!((i.image_height, i.image_width), (28, 28));
    }
}
