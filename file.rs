use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let mut file = File::open(&Path::new("test.txt")).unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content);
	let lines:Vec<&str> = (&content).lines().collect();
	println!("{:?}", lines.len());
}