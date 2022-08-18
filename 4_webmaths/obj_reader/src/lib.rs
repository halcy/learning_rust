use vector_math::{*};
use std::fs;

#[derive(Clone, Copy)]
pub struct TriData {
    pub p: [Vec3; 3],
    pub n: [Vec3; 3],
}

// Opinionated obj reader
// t/l note: opinionated means i implement only a subset and make various assumptions that may not hold in reality
pub fn parse_obj(contents: &str) -> Vec<TriData> {
    let mut vertices: Vec<Vec3> = Vec::new();
    let mut normals: Vec<Vec3> = Vec::new(); 
    let mut triangles: Vec<TriData> = Vec::new();

    for line in contents.lines() {
        let mut tokens = line.split(' ');
        let line_type = tokens.next();
        if !line_type.is_none() {
            let line_type = line_type.unwrap();
            if line_type.eq("v") {
                vertices.push(Vec3::new(
                    tokens.next().expect("Obj parse error").parse::<Scalar>().unwrap(),
                    tokens.next().expect("Obj parse error").parse::<Scalar>().unwrap(),
                    tokens.next().expect("Obj parse error").parse::<Scalar>().unwrap(),
                ));
            }

            if line_type.eq("vn") {
                normals.push(Vec3::new(
                    tokens.next().expect("Obj parse error").parse::<Scalar>().unwrap(),
                    tokens.next().expect("Obj parse error").parse::<Scalar>().unwrap(),
                    tokens.next().expect("Obj parse error").parse::<Scalar>().unwrap(),
                ));
            }

            if line_type.eq("f") {
                let mut new_tri_data = TriData {
                    p: [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
                    n: [Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)],
                };
                for idx in 0..3 {
                    let vertex_token = tokens.next().expect("Obj parse error x");
                    let mut vertex_info = vertex_token.split("/");
                    let vertex_idx = vertex_info.next().expect("Obj parse error y").parse::<usize>().unwrap() - 1;
                    vertex_info.next();
                    let normal_idx = vertex_info.next().expect("Obj parse error z").parse::<usize>().unwrap() - 1;
                    new_tri_data.p[2 - idx] = vertices[vertex_idx];
                    new_tri_data.n[2 - idx] = normals[normal_idx];
                    triangles.push(new_tri_data);
                }
            }
        }
    }
    return triangles;
}

pub fn read_obj(path: &str) -> Vec<TriData> {
    let contents = fs::read_to_string(path).expect("File read error");
    return parse_obj(&contents);
}