use model::Model;
use std::{
    fs::File,
    io::{self, BufReader},
};
use tga::{TGAColor, TGAImage};

pub mod model;
pub mod tga;

fn line(x0: usize, y0: usize, x1: usize, y1: usize, image: &mut TGAImage, color: TGAColor) {
    b_line(x0, y0, x1, y1, image, color);
}

fn xw_line(x0: usize, y0: usize, x1: usize, y1: usize, image: &mut TGAImage, color: TGAColor) {}

fn b_line(x0: usize, y0: usize, x1: usize, y1: usize, image: &mut TGAImage, color: TGAColor) {
    let mut xs = x0;
    let mut xe = x1;
    let mut ys = y0;
    let mut ye = y1;
    if (x1 as i32 - x0 as i32).abs() > (y1 as i32 - y0 as i32).abs() {
        if x0 > x1 {
            xs = x1;
            xe = x0;
            ys = y1;
            ye = y0;
        }
        for x in xs..xe + 1 {
            let t = (x - xs) as f32 / (xe + 1 - xs) as f32;
            let y = (ys as f32 * (1.0 - t) + ye as f32 * t) as usize;
            image.set(x, y, color);
        }
    } else {
        if y0 > y1 {
            xs = x1;
            xe = x0;
            ys = y1;
            ye = y0;
        }
        for y in ys..ye + 1 {
            let t = (y - ys) as f32 / (ye + 1 - ys) as f32;
            let x = (xs as f32 * (1.0 - t) + xe as f32 * t) as usize;
            image.set(x, y, color);
        }
    }
}

fn main() -> io::Result<()> {
    let model = Model::new("obj/african_head.obj")?;
    println!("nverts: {}, nfaces: {}", model.nverts(), model.nfaces());
    let width: usize = 800;
    let height: usize = 800;
    let mut image = TGAImage::new(width, height, tga::TGAFormat::RGB);
    for i in 0..model.nfaces() {
        let face = model.face(i);
        for j in 0..3 {
            let v0 = model.vert(face[j]);
            let v1 = model.vert(face[(j + 1) % 3]);
            let x0 = ((v0[0] + 1.0) * width as f32 / 2.0) as usize;
            let y0 = ((v0[1] + 1.0) * height as f32 / 2.0) as usize;
            let x1 = ((v1[0] + 1.0) * width as f32 / 2.0) as usize;
            let y1 = ((v1[1] + 1.0) * height as f32 / 2.0) as usize;
            line(x0, y0, x1, y1, &mut image, TGAColor::WHITE);
        }
    }
    image.flip_vertically();
    image.write_tga_file("output_aa.tga", true);
    Ok(())
}
