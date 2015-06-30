#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate serde;
use serde::json::{self, Value};
// file io
use std::io::prelude::*;
use std::fs::File;
#[macro_use]
extern crate itertools;
use itertools::Itertools;

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
    // let vs = Vertex{position:[1.0;3],normal:[1.0;3],texcood:[1.0;2],blendweight:[1.0;4],blendindex:[1.0;4]};
    // let serialized_vertices = json::to_string(&vs).unwrap();
    // println!("{:?}", serialized_vertices);
    let vertices: Vec<f64> = (json::from_value (data.find("vertices").unwrap().clone()) ).unwrap();
    
    let vertex_array = vertices.batching(|mut it| {
        match it.next() {
            None => None,
            Some(pos_x) => match it.next() {
                None => None,
                Some(pos_y) => match it.next() {
                    None => None,
                    Some(pos_z) => match it.next() {
                        None => None,
                        Some(nor_x) => match it.next() {
                            None => None,
                            Some(nor_y) => match it.next() {
                                None => None,
                                Some(nor_z) => match it.next() {
                                    None => None,
                                    Some(tex_x) => match it.next() {
                                        None => None,
                                        Some(tex_y) => match it.next() {
                                            None => None,
                                            Some(bw_x) => match it.next() {
                                                None => None,
                                                Some(bw_y) => match it.next() {
                                                    None => None,
                                                    Some(bw_z) => match it.next() {
                                                        None => None,
                                                        Some(bw_w) => match it.next() {
                                                            None => None,
                                                            Some(bi_x) => match it.next() {
                                                                None => None,
                                                                Some(bi_y) => match it.next() {
                                                                    None => None,
                                                                    Some(bi_z) => match it.next() {
                                                                        None => None,
                                                                        Some(bi_w) => Some(Vertex{position:[pos_x,pos_y,pos_z], normal:[nor_x,nor_y,nor_z], texcood:[tex_x, tex_y], blendweight:[bw_x,bw_y,bw_z,bw_w], blendindex:[bi_x,bi_y,bi_z,bi_w]}),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        },
                                                    },
                                                },
                                            },
                                        },
                                    },
                                },
                            },
                        },
                   },
               },
           }
       });
    for pos in vertex_array {
    	println!("{:?}", pos);
    }

 }
