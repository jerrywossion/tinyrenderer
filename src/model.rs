use std::{fs::read_to_string, io};

use nalgebra::Vector3;

#[derive(Default)]
pub struct Model {
    vertices: Vec<Vector3<f32>>,
    texture_coords: Vec<Vector3<f32>>,
    vertex_norms: Vec<Vector3<f32>>,
    faces: Vec<[Vector3<f32>; 3]>,
}

impl Model {
    pub fn new(filename: &str) -> io::Result<Self> {
        let mut model = Model::default();

        for line in read_to_string(filename)?.lines() {
            // parse vertices
            if line.starts_with("v ") {
                let parsed_vertex: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|fstr| fstr.parse::<f32>().ok())
                    .collect();
                if parsed_vertex.len() != 3 {
                    continue;
                }
                let mut vertex = Vector3::zeros();
                for i in 0..3 {
                    vertex[i] = parsed_vertex[i];
                }
                model.vertices.push(vertex);
            } else if line.starts_with("f ") {
                // parse faces
                let parsed_face: Vec<Vector3<f32>> = line
                    .split_whitespace()
                    .filter_map(|fstr| -> Option<Vector3<f32>> {
                        let parts: Vec<usize> = fstr
                            .split('/')
                            .filter_map(|istr| istr.parse::<usize>().ok())
                            .collect();
                        if parts.len() != 3 {
                            return None;
                        } else {
                            return Some(Vector3::new(
                                (parts[0] - 1) as f32,
                                (parts[1] - 1) as f32,
                                (parts[2] - 1) as f32,
                            ));
                        }
                    })
                    .collect();
                if parsed_face.len() != 3 {
                    continue;
                }
                model
                    .faces
                    .push([parsed_face[0], parsed_face[1], parsed_face[2]]);
            } else if line.starts_with("vt ") {
                // parse texture coords
                let parsed_texture_coord: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|fstr| fstr.parse::<f32>().ok())
                    .collect();
                if parsed_texture_coord.len() != 3 {
                    continue;
                }
                let mut texture_coord = Vector3::zeros();
                for i in 0..3 {
                    texture_coord[i] = parsed_texture_coord[i];
                }
                model.texture_coords.push(texture_coord);
            } else if line.starts_with("vn ") {
                // parse vertex norms
                let parsed_vertex_norm: Vec<f32> = line
                    .split_whitespace()
                    .filter_map(|fstr| fstr.parse::<f32>().ok())
                    .collect();
                if parsed_vertex_norm.len() != 3 {
                    continue;
                }
                let mut vertex_norm = Vector3::zeros();
                for i in 0..3 {
                    vertex_norm[i] = parsed_vertex_norm[i];
                }
                model.vertex_norms.push(vertex_norm);
            }
        }
        Ok(model)
    }

    pub fn vertex_cnt(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_cnt(&self) -> usize {
        self.faces.len()
    }

    pub fn face(&self, idx: usize) -> &[Vector3<f32>; 3] {
        &self.faces[idx]
    }

    pub fn vertex(&self, idx: usize) -> &Vector3<f32> {
        &self.vertices[idx]
    }

    pub fn texture_coord(&self, idx: usize) -> &Vector3<f32> {
        &self.texture_coords[idx]
    }

    pub fn vertex_norm(&self, idx: usize) -> &Vector3<f32> {
        &self.vertex_norms[idx]
    }
}
