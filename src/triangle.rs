use std::cmp::{max, min};

use glam::{Mat2, Vec2, Vec3};

use crate::{
    line::draw_line,
    tga::{TGAColor, TGAImage},
};

fn barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2, u: &mut f32, v: &mut f32) {
    let c = c - a;
    let b = b - a;
    let p = p - a;
    let cc = c.dot(c);
    let bc = b.dot(c);
    let pc = c.dot(p);
    let bb = b.dot(b);
    let pb = b.dot(p);
    let d = cc * bb - bc * bc;
    *u = (bb * pc - bc * pb) / d;
    *v = (cc * pb - bc * pc) / d;
}

fn inside(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let mut u: f32 = 0.0;
    let mut v: f32 = 0.0;
    barycentric(p, a, b, c, &mut u, &mut v);
    u >= 0.0 && v >= 0.0 && (u + v) <= 1.0
}

pub fn draw_triangle(
    a: Vec3,
    b: Vec3,
    c: Vec3,
    zbuffer: &mut Vec<f32>,
    image: &mut TGAImage,
    texture: &TGAImage,
    vts: &Vec<Vec2>,
    antialiasing: bool,
) {
    // draw_line(a, b, image, color, antialiasing);
    // draw_line(b, c, image, color, antialiasing);
    // draw_line(c, a, image, color, antialiasing);

    let xs = min(min(a.x as usize, b.x as usize), c.x as usize);
    let ys = min(min(a.y as usize, b.y as usize), c.y as usize);
    let xe = max(max(a.x as usize, b.x as usize), c.x as usize);
    let ye = max(max(a.y as usize, b.y as usize), c.y as usize);

    for y in ys..ye + 1 {
        for x in xs..xe + 1 {
            let mut u: f32 = 0.0;
            let mut v: f32 = 0.0;
            barycentric(
                Vec2 {
                    x: x as f32,
                    y: y as f32,
                },
                Vec2 { x: a.x, y: a.y },
                Vec2 { x: b.x, y: b.y },
                Vec2 { x: c.x, y: c.y },
                &mut u,
                &mut v,
            );
            let inside = u >= 0.0 && v >= 0.0 && (u + v) <= 1.0;
            let z = u * a.z + v * b.z + (1.0 - u - v) * c.z;
            let idx = x + image.get_width() * y;
            let vt_coord = vts[0] + (vts[2] - vts[0]) * u + (vts[1] - vts[0]) * v;
            if inside && z > zbuffer[idx] {
                image.set(
                    x as usize,
                    y as usize,
                    texture.get(
                        (vt_coord.x * texture.get_width() as f32) as usize,
                        (vt_coord.y * texture.get_width() as f32) as usize,
                    ),
                );
                zbuffer[idx] = z;
            }
        }
    }
}
