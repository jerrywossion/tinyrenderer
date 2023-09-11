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
    let mut zbuffer: Vec<f32> = vec![f32::MIN; width * height];
    let mut texture = TGAImage::new(width, height, tga::TGAFormat::RGB);
    texture.read_tga_file("obj/african_head_diffuse.tga")?;
    texture.flip_vertically();
    texture.scale(width, height);
    for i in 0..model.nfaces() {
        let face = model.face(i);

        let mut vs: Vec<Vec3> = vec![];
        let mut ovs: Vec<Vec3> = vec![];
        let mut vts: Vec<Vec2> = vec![];
        for j in 0..3 {
            let v = model.vert(face[j].x as usize);
            let x = (v[0] + 1.0) * width as f32 / 2.0;
            let y = (v[1] + 1.0) * height as f32 / 2.0;
            vs.push(Vec3 { x, y, z: v[2] });
            ovs.push(v);
            vts.push(model.texture(face[j].y as usize));
        }
        let n = ((ovs[2] - ovs[1]).cross(ovs[1] - ovs[0])).normalize();
        let intensity = (n * light_dir).z;
        if intensity > 0.0 {
            draw_triangle(
                vs[0],
                vs[1],
                vs[2],
                &mut zbuffer,
                &mut image,
                &texture,
                &vts,
                intensity,
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
    _ = draw_mesh();
}
