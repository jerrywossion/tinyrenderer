use std::cmp::{max, min};

use glam::Vec2;

use crate::{
    line::draw_line,
    tga::{TGAColor, TGAImage},
};

fn inside(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
    let c = c - a;
    let b = b - a;
    let p = p - a;
    let cc = c.dot(c);
    let bc = b.dot(c);
    let pc = c.dot(p);
    let bb = b.dot(b);
    let pb = b.dot(p);
    let d = cc * bb - bc * bc;
    let u = (bb * pc - bc * pb) / d;
    let v = (cc * pb - bc * pc) / d;
    u >= 0.0 && v >= 0.0 && (u + v) < 1.0
}

pub fn draw_triangle(
    a: Vec2,
    b: Vec2,
    c: Vec2,
    image: &mut TGAImage,
    color: TGAColor,
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
            if inside(
                Vec2 {
                    x: x as f32,
                    y: y as f32,
                },
                a,
                b,
                c,
            ) {
                image.set(x as usize, y as usize, color);
            }
        }
    }
}
