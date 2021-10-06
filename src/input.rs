mod input_file;
mod input_image;

pub use input_file::InputFile;
pub use input_image::*;

pub fn main() {
    println!("Generating files...");
    let f = InputFile::new("data/t10k-images-idx3-ubyte");
    f.render_image_files_parallel();
    println!("Done!");
}
