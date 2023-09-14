use std::cmp::{max, min};

use nalgebra::{Matrix4x3, Vector2, Vector3, Vector4};

use crate::tga::{TGAColor, TGAImage};

struct Triangle<'a> {
    a: &'a Vector2<f32>,
    b: &'a Vector2<f32>,
    c: &'a Vector2<f32>,
}

impl<'a> Triangle<'a> {
    fn new(a: &'a Vector2<f32>, b: &'a Vector2<f32>, c: &'a Vector2<f32>) -> Self {
        Self { a, b, c }
    }

    fn barycentric(&self, p: &Vector2<f32>, u: &mut f32, v: &mut f32) {
        let c = self.c - self.a;
        let b = self.b - self.a;
        let p = p - self.a;
        let cc = c.dot(&c);
        let bc = b.dot(&c);
        let pc = c.dot(&p);
        let bb = b.dot(&b);
        let pb = b.dot(&p);
        let d = cc * bb - bc * bc;
        *u = (bb * pc - bc * pb) / d;
        *v = (cc * pb - bc * pc) / d;
    }

    fn contains(&self, p: &Vector2<f32>) -> bool {
        let mut u: f32 = 0.0;
        let mut v: f32 = 0.0;
        self.contains_with_uv(p, &mut u, &mut v)
    }

    fn contains_with_uv(&self, p: &Vector2<f32>, u: &mut f32, v: &mut f32) -> bool {
        self.barycentric(p, u, v);
        *u >= 0.0 && *v >= 0.0 && (*u + *v) <= 1.0
    }
}

pub fn draw_triangle(
    vertices: &Matrix4x3<f32>,
    texture_coords: &Vec<Vector3<f32>>,
    vertex_norms: &Vec<Vector3<f32>>,
    light_dir: &Vector3<f32>,
    image: &mut TGAImage,
    zbuffer: &mut Vec<f32>,
    texture: &TGAImage,
    color: &TGAColor,
    use_texture: bool,
) {
    let a = vertices.column(0);
    let b = vertices.column(1);
    let c = vertices.column(2);
    let xs = min(min(a.x as usize, b.x as usize), c.x as usize);
    let ys = min(min(a.y as usize, b.y as usize), c.y as usize);
    let xe = max(max(a.x as usize, b.x as usize), c.x as usize);
    let ye = max(max(a.y as usize, b.y as usize), c.y as usize);

    let triangle = Triangle {
        a: &a.fixed_rows::<2>(0).into(),
        b: &b.fixed_rows::<2>(0).into(),
        c: &c.fixed_rows::<2>(0).into(),
    };

    for y in ys..ye + 1 {
        for x in xs..xe + 1 {
            // calculate u, v & check if inside
            let mut u: f32 = 0.0;
            let mut v: f32 = 0.0;
            let inside =
                triangle.contains_with_uv(&Vector2::new(x as f32, y as f32), &mut u, &mut v);
            let w = 1.0 - u - v;
            // calculate z
            let z = u * a.z + v * b.z + (1.0 - u - v) * c.z;

            let idx = x + image.get_width() * y;
            if idx >= zbuffer.len() {
                return;
            }
            // interop norm
            let norm = Vector3::new(
                vertex_norms[0].x * w + vertex_norms[2].x * u + vertex_norms[1].x * v,
                vertex_norms[0].y * w + vertex_norms[2].y * u + vertex_norms[1].y * v,
                vertex_norms[0].z * w + vertex_norms[2].z * u + vertex_norms[1].z * v,
            );
            // calculate light intensity
            let intensity = norm.dot(&light_dir);
            if intensity > 0.0 && inside && z > zbuffer[idx] {
                let color = if use_texture {
                    // calculate texture uv coords
                    let texture_coord = texture_coords[0]
                        + (texture_coords[2] - texture_coords[0]) * u
                        + (texture_coords[1] - texture_coords[0]) * v;
                    texture.get(
                        (texture_coord.x * texture.get_width() as f32) as usize,
                        (texture_coord.y * texture.get_height() as f32) as usize,
                    )
                } else {
                    color.to_owned()
                }
                .get_color(intensity);
                image.set(x as usize, y as usize, color);
                zbuffer[idx] = z;
            }
        }
    }
}
