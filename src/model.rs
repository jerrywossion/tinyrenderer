use std::{fs::read_to_string, io};

use glam::Vec3;

pub struct Model {
    verts: Vec<Vec3>,
    faces: Vec<Vec<usize>>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut verts = Vec::<Vec3>::new();
        let mut faces = Vec::<Vec<usize>>::new();
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
                let fvs: Vec<usize> = line
                    .split_whitespace()
                    .filter_map(|fstr| -> Option<usize> {
                        let parts: Vec<usize> = fstr
                            .split('/')
                            .filter_map(|istr| istr.parse::<usize>().ok())
                            .collect();
                        if parts.len() != 3 {
                            return None;
                        } else {
                            return Some(parts[0] - 1);
                        }
                    })
                    .collect();
                faces.push(fvs);
            }
        }
        Ok(Self { verts, faces })
    }

    pub fn nverts(&self) -> usize {
        self.verts.len()
    }

    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    pub fn face(&self, idx: usize) -> &Vec<usize> {
        &self.faces[idx]
    }

    pub fn vert(&self, idx: usize) -> Vec3 {
        self.verts[idx]
    }
}
