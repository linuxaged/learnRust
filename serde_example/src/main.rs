#![feature(custom_derive, plugin,step_by)]
#![plugin(serde_macros)]
extern crate serde;
use serde::json::{self, Value};
// file io
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("./orc.c3t").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let data: Value = json::from_str(&s).unwrap();
    let meshes = data.find("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();

    #[derive(Copy, Clone, Serialize, Deserialize, Display, Debug)]
    struct Vertex {
        position:   [f64; 3],
        normal:     [f64; 3],
        texcood:    [f64; 2],
        blendweight:[f64; 4],
        blendindex: [f64; 4]
    }
    // let vs = Vertex{position:[1.0;3],normal:[1.0;3],texcood:[1.0;2],blendweight:[1.0;4],blendindex:[1.0;4]};
    // let serialized_vertices = json::to_string(&vs).unwrap();
    // println!("{:?}", serialized_vertices);
    let vertices: Vec<f64> = (json::from_value(mesh.get("vertices").unwrap().clone()) ).unwrap();
    
    let mut vertex_array:Vec<Vertex> = Vec::<Vertex>::new();
    
    for i in (0..vertices.len()).step_by(16) {
        let vertex = Vertex{
            position:[vertices[i+0], vertices[i+1],vertices[i+2]],
            normal:[vertices[i+3],vertices[i+4],vertices[i+5]],
            texcood:[vertices[i+6],vertices[i+7]],
            blendweight:[vertices[i+8], vertices[i+9],vertices[i+10],vertices[i+11]],
            blendindex:[vertices[i+12], vertices[i+13],vertices[i+14],vertices[i+15]]
        };
        vertex_array.push(vertex);
    }

    for pos in vertex_array {
    	println!("{:?}", pos);
    }

 }
