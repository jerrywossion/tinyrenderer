use model::Model;
use std::io::{self};
use tga::{TGAColor, TGAImage};

use crate::line::draw_line;

pub mod line;
pub mod model;
pub mod tga;

fn draw_mesh() -> io::Result<()> {
    let model = Model::new("obj/african_head.obj")?;
    let width: usize = 800;
    let height: usize = 800;
    let mut image = TGAImage::new(width, height, tga::TGAFormat::RGB);
    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vert(face[j]);
            let v1 = model.vert(face[(j + 1) % 3]);
            let x0 = ((v0[0] + 1.0) * width as f32 / 2.0) as i32;
            let y0 = ((v0[1] + 1.0) * height as f32 / 2.0) as i32;
            let x1 = ((v1[0] + 1.0) * width as f32 / 2.0) as i32;
            let y1 = ((v1[1] + 1.0) * height as f32 / 2.0) as i32;
            draw_line(x0, y0, x1, y1, &mut image, TGAColor::WHITE);
        }
    }
    image.flip_vertically();
    image.write_tga_file("output.tga", true)?;

    Ok(())
}

fn main() {
    _ = draw_mesh();
}
