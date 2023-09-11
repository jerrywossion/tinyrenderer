use std::{fs::read_to_string, io};

use glam::{Vec2, Vec3};

pub struct Model {
    verts: Vec<Vec3>,
    vts: Vec<Vec2>,
    faces: Vec<Vec<Vec3>>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut verts = Vec::<Vec3>::new();
        let mut vts = Vec::<Vec2>::new();
        let mut faces = Vec::<Vec<Vec3>>::new();
        for line in read_to_string(filename)?.lines() {
            if line.starts_with("v ") {
                let vs: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|fstr| fstr.parse::<f32>().ok())
                    .collect();
                if vs.len() != 3 {
                    continue;
                }
                let mut vert = Vec3::default();
                for i in 0..3 {
                    vert[i] = vs[i];
                }
                verts.push(vert);
            } else if line.starts_with("f ") {
                let fvs: Vec<Vec3> = line
                    .split_whitespace()
                    .filter_map(|fstr| -> Option<Vec3> {
                        let parts: Vec<usize> = fstr
                            .split('/')
                            .filter_map(|istr| istr.parse::<usize>().ok())
                            .collect();
                        if parts.len() != 3 {
                            return None;
                        } else {
                            return Some(Vec3 {
                                x: (parts[0] - 1) as f32,
                                y: (parts[1] - 1) as f32,
                                z: parts[2] as f32,
                            });
                        }
                    })
                    .collect();
                faces.push(fvs);
            } else if line.starts_with("vt ") {
                let vt_s: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|fstr| fstr.parse::<f32>().ok())
                    .collect();
                if vt_s.len() != 3 {
                    continue;
                }
                let mut vt = Vec2::default();
                for i in 0..2 {
                    vt[i] = vt_s[i];
                }
                vts.push(vt);
            }
        }
        Ok(Self { verts, vts, faces })
    }

    pub fn nverts(&self) -> usize {
        self.verts.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn face(&self, idx: usize) -> &Vec<Vec3> {
        &self.faces[idx]
    }

    pub fn vert(&self, idx: usize) -> Vec3 {
        self.verts[idx]
    }

    pub fn texture(&self, idx: usize) -> Vec2 {
        self.vts[idx]
    }
}
