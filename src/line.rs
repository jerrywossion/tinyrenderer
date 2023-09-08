use crate::tga::{TGAColor, TGAImage};

pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    // b_line(x0, y0, x1, y1, image, color);
    // xw_line(x0, y0, x1, y1, image, color);
    xw_line_jerryw(x0, y0, x1, y1, image, color);
}
fn ipart(x: f32) -> f32 {
    x.floor()
}

fn round(x: f32) -> f32 {
    ipart(x + 0.5)
}

fn fpart(x: f32) -> f32 {
    x - ipart(x)
}

fn rfpart(x: f32) -> f32 {
    1.0 - fpart(x)
}

fn get_color(color: &TGAColor, frac: f32) -> TGAColor {
    TGAColor {
        r: (color.r as f32 * frac) as u8,
        g: (color.g as f32 * frac) as u8,
        b: (color.b as f32 * frac) as u8,
        a: (color.a as f32 * frac) as u8,
    }
}

fn xw_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut xs = x0;
    let mut xe = x1;
    let mut ys = y0;
    let mut ye = y1;
    let is_steep = (x1 as i32 - x0 as i32).abs() < (y1 as i32 - y0 as i32).abs();
    if is_steep {
        let mut t = xs;
        xs = ys;
        ys = t;
        t = xe;
        xe = ye;
        ye = t;
    }
    if xs > xe {
        let mut t = xs;
        xs = xe;
        xe = t;
        t = ys;
        ys = ye;
        ye = t;
    }
    let dx = xe - xs;
    let dy = ye - ys;
    let mut gradient: f32 = 0.0;
    if dx == 0 {
        gradient = 1.0;
    } else {
        gradient = dy as f32 / dx as f32;
    }
    let mut xend = xs as f32;
    let mut yend = ys as f32 + gradient * (xend - xs as f32);
    let mut xgap = rfpart(xs as f32 + 0.5);
    let mut xpxl1 = xend;
    let mut ypxl1 = ipart(yend);
    if is_steep {
        image.set(
            ypxl1 as usize,
            xpxl1 as usize,
            get_color(&color, rfpart(yend) * xgap),
        );
        image.set(
            ypxl1 as usize + 1,
            xpxl1 as usize,
            get_color(&color, fpart(yend) * xgap),
        );
    } else {
        image.set(
            xpxl1 as usize,
            ypxl1 as usize,
            get_color(&color, rfpart(yend) * xgap),
        );
        image.set(
            xpxl1 as usize,
            ypxl1 as usize + 1,
            get_color(&color, fpart(yend) * xgap),
        );
    }
    let mut intery = yend + gradient;

    xend = xe as f32;
    yend = ye as f32 + gradient * (xend - xe as f32);
    xgap = fpart(xe as f32 + 0.5);
    let mut xpxl2 = xend;
    let mut ypxl2 = ipart(yend);
    if is_steep {
        image.set(
            ypxl2 as usize,
            xpxl2 as usize,
            get_color(&color, rfpart(yend) * xgap),
        );
        image.set(
            ypxl2 as usize + 1,
            xpxl2 as usize,
            get_color(&color, fpart(yend) * xgap),
        );
    } else {
        image.set(
            xpxl2 as usize,
            ypxl2 as usize,
            get_color(&color, rfpart(yend) * xgap),
        );
        image.set(
            xpxl2 as usize,
            ypxl2 as usize + 1,
            get_color(&color, rfpart(yend) * xgap),
        );
    }
    for x in (xpxl1 as usize + 1)..(xpxl2 as usize - 1) {
        if is_steep {
            image.set(ipart(intery) as usize, x, get_color(&color, rfpart(intery)));
            image.set(
                ipart(intery) as usize + 1,
                x,
                get_color(&color, fpart(intery)),
            );
        } else {
            image.set(x, ipart(intery) as usize, get_color(&color, rfpart(intery)));
            image.set(
                x,
                ipart(intery) as usize + 1,
                get_color(&color, fpart(intery)),
            );
        }
        intery += gradient;
    }
}

fn xw_line_jerryw(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut xs = x0;
    let mut xe = x1;
    let mut ys = y0;
    let mut ye = y1;
    let is_steep = (x1 as i32 - x0 as i32).abs() < (y1 as i32 - y0 as i32).abs();
    if is_steep {
        if y0 > y1 {
            xs = y1;
            xe = y0;
            ys = x1;
            ye = x0;
        } else {
            xs = y0;
            xe = y1;
            ys = x0;
            ye = x1;
        }
    } else {
        if x0 > x1 {
            xs = x1;
            xe = x0;
            ys = y1;
            ye = y0;
        }
    }
    if xe == xs {
        for y in ys..ye {
            image.set(xs as usize, y as usize, color);
        }
        return;
    }
    let k = (ye - ys) as f32 / (xe - xs) as f32;
    let b = ys as f32 - k * xs as f32;
    for x in xs..xe + 1 {
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

fn b_line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut xs = x0;
    let mut xe = x1;
    let mut ys = y0;
    let mut ye = y1;
    let is_steep = (x1 as i32 - x0 as i32).abs() < (y1 as i32 - y0 as i32).abs();
    if is_steep {
        if y0 > y1 {
            xs = y1;
            xe = y0;
            ys = x1;
            ye = x0;
        } else {
            xs = y0;
            xe = y1;
            ys = x0;
            ye = x1;
        }
    } else {
        if x0 > x1 {
            xs = x1;
            xe = x0;
            ys = y1;
            ye = y0;
        }
    }
    for x in xs..xe + 1 {
        let t = (x - xs) as f32 / (xe + 1 - xs) as f32;
        let y = (ys as f32 * (1.0 - t) + ye as f32 * t) as i32;
        if is_steep {
            image.set(y as usize, x as usize, color);
        } else {
            image.set(x as usize, y as usize, color);
        }
    }
}
