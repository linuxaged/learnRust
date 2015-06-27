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
    let meshes = data.find("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();
    let length = mesh.len();
    let mut v = vec![0f64; length];
    v = (json::from_value (mesh.get("vertices").unwrap().clone()) ).unwrap();
    for pos in v {
    	println!("{}", pos);
    }
 }
