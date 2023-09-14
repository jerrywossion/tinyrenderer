use model::Model;
use nalgebra::{Matrix4, Matrix4x3, Vector3, Vector4};
use std::io::{self};
use tga::{TGAColor, TGAImage};
use triangle::draw_triangle;

pub mod line;
pub mod model;
pub mod tga;
pub mod triangle;

fn render_model(model_file: &str, texture_file: &str, image: &mut TGAImage) -> io::Result<()> {
    let model = Model::new(model_file)?;
    let width = image.get_width();
    let height = image.get_height();

    let light_dir = Vector3::new(0.0, 0.0, 1.0);

    let mut z_buffer: Vec<f32> = vec![f32::MIN; width * height];
    // read texture
    let mut texture = TGAImage::new(width, height, tga::TGAFormat::RGB);
    texture.read_tga_file(texture_file);
    texture.flip_vertically();
    texture.scale(width, height);

    let depth = 255.0;
    #[rustfmt::skip]
    let vx = (width as f32) / 8.0;
    let vy = (height as f32) / 8.0;
    let vw = (width as f32) * 3.0 / 4.0;
    let vh = (height as f32) * 3.0 / 4.0;
    let view_port = Matrix4::new(
        vw / 2.0,
        0.0,
        0.0,
        vx + vw / 2.0,
        0.0,
        vh / 2.0,
        0.0,
        vy + vh / 2.0,
        0.0,
        0.0,
        depth / 2.0,
        depth / 2.0,
        0.0,
        0.0,
        0.0,
        1.0,
    );
    #[rustfmt::skip]
    let projection = Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, -1.0/3.0, 1.0,
    );
    // render vertices
    for i in 0..model.face_cnt() {
        let face = model.face(i);

        let mut vertices = Matrix4x3::identity();
        let mut texture_coords: Vec<Vector3<f32>> = vec![];
        let mut vertex_norms: Vec<Vector3<f32>> = vec![];

        for j in 0..3 {
            let vertex = model.vertex(face[j].x as usize);
            vertices.set_column(j, &vertex.insert_row(3, 1.0));
            texture_coords.push(model.texture_coord(face[j].y as usize).to_owned());
            vertex_norms.push(model.vertex_norm(face[j].z as usize).to_owned());
        }

        let screen_vertices = view_port * projection * vertices;

        draw_triangle(
            &screen_vertices,
            &texture_coords,
            &vertex_norms,
            &light_dir,
            image,
            &mut z_buffer,
            &texture,
            &TGAColor::WHITE,
            true,
        );
    }
    Ok(())
}

fn main() {
    let width: usize = 800;
    let height: usize = 800;
    let mut image = TGAImage::new(width, height, tga::TGAFormat::RGB);
    _ = render_model(
        "obj/african_head.obj",
        "obj/african_head_diffuse.tga",
        &mut image,
    );

    image.flip_vertically();
    _ = image.write_tga_file("output.tga", true);
}
