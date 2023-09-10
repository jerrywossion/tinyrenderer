use glam::{Vec2, Vec3};
use model::Model;
use std::io::{self};
use tga::{TGAColor, TGAImage};

use triangle::draw_triangle;

pub mod line;
pub mod model;
pub mod tga;
pub mod triangle;

fn draw_mesh() -> io::Result<()> {
    let model = Model::new("obj/african_head.obj")?;
    let width: usize = 800;
    let height: usize = 800;
    let mut image = TGAImage::new(width, height, tga::TGAFormat::RGB);
    let light_dir = Vec3::from_array([0.0, 0.0, -1.0]);
    for i in 0..model.nfaces() {
        let face = model.face(i);

        let mut vs: Vec<Vec2> = vec![];
        let mut wc: Vec<Vec3> = vec![];
        for j in 0..3 {
            let v = model.vert(face[j]);
            let x = (v[0] + 1.0) * width as f32 / 2.0;
            let y = (v[1] + 1.0) * height as f32 / 2.0;
            vs.push(Vec2 { x, y });
            wc.push(v);
        }
        let n = ((wc[2] - wc[1]).cross(wc[1] - wc[0])).normalize();
        let intensity = (n * light_dir).z;
        if intensity > 0.0 {
            draw_triangle(
                vs[0],
                vs[1],
                vs[2],
                &mut image,
                TGAColor::WHITE.get_color(intensity),
                false,
            );
        }
    }
    image.flip_vertically();
    image.write_tga_file("output.tga", true)?;

    Ok(())
}

fn main() {
    let width: usize = 200;
    let height: usize = 200;
    let mut image = TGAImage::new(width, height, tga::TGAFormat::RGB);
    draw_triangle(
        Vec2::from_array([10.0, 70.0]),
        Vec2::from_array([50.0, 160.0]),
        Vec2::from_array([70.0, 80.0]),
        &mut image,
        TGAColor::WHITE,
        false,
    );
    image.flip_vertically();
    _ = image.write_tga_file("triangle.tga", true);

    _ = draw_mesh();
}
