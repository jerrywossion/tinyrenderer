use tga::{TGAColor, TGAFormat, TGAImage};

pub mod tga;

fn main() {
    let mut image = TGAImage::new(100, 100, TGAFormat::RGB);
    image.set(20, 20, TGAColor::WHITE);
    image.write_tga_file("output.tga", true);
}
