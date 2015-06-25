// serde
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
    let obj = data.as_object().unwrap();
    let meshes = obj.get("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();
    let vertices: Vec<json::Value> = mesh.get("vertices").unwrap().as_array().unwrap().to_vec();
    let vertex_array: Vec<f64> = vertices.into_iter().map (|value| json::from_value (value) .unwrap()) .collect();
    for pos in vertex_array {
    	println!("{}", pos);
    }
 }
