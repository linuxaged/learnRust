// serde
extern crate serde;
use serde::json::{self, Value};
// file io
use std::io::prelude::*;
use std::fs::File;
//
use std::collections::BTreeMap;

fn main() {
	let mut f = File::open("./orc.c3t").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let data: Value = json::from_str(&s).unwrap();
    let meshes = data.find("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();
    let vertices: Vec<f64> = (json::from_value (mesh.get("vertices").unwrap()) ).unwrap();
    for pos in vertices {
    	println!("{}", pos);
    }
 }
