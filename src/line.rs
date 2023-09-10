use glam::Vec2;

use crate::tga::{TGAColor, TGAImage};

pub fn draw_line(a: Vec2, b: Vec2, image: &mut TGAImage, color: TGAColor, antialiasing: bool) {
    if antialiasing {
        xw_line_jerryw(a, b, image, color);
    } else {
        b_line(a, b, image, color);
    }
}

fn get_color(color: &TGAColor, frac: f32) -> TGAColor {
    let has_alpha = true;
    if has_alpha {
        TGAColor {
            a: (color.a as f32 * frac) as u8,
            ..*color
        }
    } else {
        TGAColor {
            r: (color.r as f32 * frac) as u8,
            g: (color.g as f32 * frac) as u8,
            b: (color.b as f32 * frac) as u8,
            a: (color.a as f32 * frac) as u8,
        }
    }
}

fn xw_line_jerryw(a: Vec2, b: Vec2, image: &mut TGAImage, color: TGAColor) {
    let mut xs = a.x;
    let mut xe = b.x;
    let mut ys = a.y;
    let mut ye = b.y;
    let is_steep = (xs - xe).abs() < (ys - ye).abs();
    if is_steep {
        if a.y > b.y {
            xs = b.y;
            xe = a.y;
            ys = b.x;
            ye = a.x;
        } else {
            xs = a.y;
            xe = b.y;
            ys = a.x;
            ye = b.x;
        }
    } else {
        if a.x > b.x {
            xs = b.x;
            xe = a.x;
            ys = b.y;
            ye = a.y;
        }
    }
    if xe == xs {
        for y in ys as usize..ye as usize {
            image.set(xs as usize, y, color);
        }
        return;
    }
    let k = (ye - ys) as f32 / (xe - xs) as f32;
    let b = ys as f32 - k * xs as f32;
    for x in xs as usize..(xe + 1.0) as usize {
        let y = k * x as f32 + b;
        let yu = y.ceil();
        let yd = y.floor();
        let fu = 1.0 - (yu - y);
        let fd = 1.0 - (y - yd);
        let coloru = TGAColor {
            r: (color.r as f32 * fu) as u8,
            g: (color.g as f32 * fu) as u8,
            b: (color.b as f32 * fu) as u8,
            a: (color.a as f32 * fu) as u8,
        };
        let colord = TGAColor {
            r: (color.r as f32 * fd) as u8,
            g: (color.g as f32 * fd) as u8,
            b: (color.b as f32 * fd) as u8,
            a: (color.a as f32 * fd) as u8,
        };
        if is_steep {
            image.set(yu as usize, x as usize, coloru);
            image.set(yd as usize, x as usize, colord);
        } else {
            image.set(x as usize, yu as usize, coloru);
            image.set(x as usize, yd as usize, colord);
        }
    }
}

fn b_line(a: Vec2, b: Vec2, image: &mut TGAImage, color: TGAColor) {
    let mut xs = a.x;
    let mut xe = b.x;
    let mut ys = a.y;
    let mut ye = b.y;
    let is_steep = (xs - xe).abs() < (ys - ye).abs();
    if is_steep {
        if a.y > b.y {
            xs = b.y;
            xe = a.y;
            ys = b.x;
            ye = a.x;
        } else {
            xs = a.y;
            xe = b.y;
            ys = a.x;
            ye = b.x;
        }
    } else {
        if a.x > b.x {
            xs = b.x;
            xe = a.x;
            ys = b.y;
            ye = a.y;
        }
    }
    for x in xs as usize..(xe + 1.0) as usize {
        let t = (x as f32 - xs) / (xe + 1.0 - xs);
        let y = (ys as f32 * (1.0 - t) + ye as f32 * t) as i32;
        if is_steep {
            image.set(y as usize, x as usize, color);
        } else {
            image.set(x as usize, y as usize, color);
        }
    }
}
