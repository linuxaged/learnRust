#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;
use serde::json::{self, Value};
// file io
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("./test.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);
    let data: Value = json::from_str(&s).unwrap();
    #[derive(Copy, Clone, Serialize, Deserialize, Display, Debug)]
    struct Vertex {
        position:   [f64; 3],
        normal:     [f64; 3],
        texcood:    [f64; 2],
        blendweight:[f64; 4],
        blendindex: [f64; 4]
    }
    let vertices: Vec<Vertex> = (json::from_value (data.find("vertices").unwrap().clone()) ).unwrap();
    for pos in vertices {
    	println!("{:?}", pos);
    }

 }
