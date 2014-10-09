use std::io::File;

fn main() {
	println!("i will create a file name test.txt");
	let mut file = File::create(&Path::new("test.txt"));
	match file.write(b"hello, world\n") {
		Ok(()) => (),
		Err(e) => println!("fail to write to file test.txt:{}", e),
	}
}