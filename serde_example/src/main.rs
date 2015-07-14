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

    #[derive(Copy, Clone, Serialize, Display, Debug)]
    struct Vertex {
        position:   [f64; 3],
        normal:     [f64; 3],
        texcoord:    [f64; 2],
        blendweight:[f64; 4],
        blendindex: [f64; 4]
    }
    #[derive(Serialize, Deserialize,Display)]
    struct Vertices {
        vs: Vec<Vertex>
    }

    impl serde::Deserialize for Vertex {
        #[inline]
        fn deserialize<D>(deserializer: &mut D) -> Result<Vertex, D::Error>
            where D: serde::Deserializer {

            let vertices: [f64; 16] = try!(serde::Deserialize::deserialize(deserializer));

            Ok(Vertex {
                position:[vertices[0], vertices[1],vertices[2]],
                normal:[vertices[3], vertices[4], vertices[5]],
                texcoord:[vertices[6], vertices[7]],
                blendweight:[vertices[8], vertices[9], vertices[10], vertices[11]],
                blendindex:[vertices[12], vertices[13],vertices[14], vertices[15]]
            })
        }
    }
    // let vs = Vertex{position:[1.0;3],normal:[1.0;3],texcoord:[1.0;2],blendweight:[1.0;4],blendindex:[1.0;4]};
    // let serialized_vertices = json::to_string(&vs).unwrap();
    // println!("{:?}", serialized_vertices);
    let vertices:Vertices = (json::from_value(mesh.get("vertices").unwrap().clone()) ).unwrap();

    // let mut vertex_array:Vec<Vertex> = Vec::<Vertex>::new();

    // for i in (0..vertices.len()).step_by(16) {
    //     let vertex = Vertex{
    //         position:[vertices[i+0], vertices[i+1],vertices[i+2]],
    //         normal:[vertices[i+3],vertices[i+4],vertices[i+5]],
    //         texcoord:[vertices[i+6],vertices[i+7]],
    //         blendweight:[vertices[i+8], vertices[i+9],vertices[i+10],vertices[i+11]],
    //         blendindex:[vertices[i+12], vertices[i+13],vertices[i+14],vertices[i+15]]
    //     };
    //     vertex_array.push(vertex);
    // }

    // let val: Value = json::from_str(r#"{"floats":[1.0,2.0,3.0,4.0,
    //     5.0,6.0,7.0,8.0,
    //     9.0,10.0,11.0,12.0,
    //     13.0,14.0,15.0,16.0]}"#).unwrap();
    // let fs:Vertex = json::from_value(val.find("floats").unwrap().clone() ).unwrap() ;
    // println!("{:?}", fs);

    // for pos in vertex_array {
    // 	println!("{:?}", pos);
    // }

 }
