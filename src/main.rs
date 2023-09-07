use tga::{TGAColor, TGAFormat, TGAImage};

mod tga;

fn main() {
    let mut image = TGAImage::new(100, 100, TGAFormat::RGB);
    image.set(20, 20, TGAColor::RED);
    // image.set(21, 20, TGAColor::RED);
    // image.set(20, 21, TGAColor::RED);
    // image.set(21, 21, TGAColor::RED);
    image.flip_horizontally();
    // image.flip_vertically();
    image.write_tga_file("output.tga", true);
    let mut image2 = TGAImage::new(0, 0, TGAFormat::GRAYSCALE);
    image2.read_tga_file("output.tga");
    image2.flip_horizontally();
    image2.write_tga_file("output2.tga", true);
}
