use std::io;
use std::vec::Vec;

static key: &'static str = "AaCcGgTtUuMmRrWwSsYyKkVvHhDdBbNn";
static cmp: &'static str = "TTGGCCAAAAKKYYWWSSRRMMBBDDHHVVNN";

fn revcomp(buffer: &mut Vec<u8>,split: &Vec<int>) {
    let index = 0;
    // TODO
    // itor over split, malloc mem for splits
    for i in range(begin, end) {
        index = key.find();
        buffer[i] = cmp[index];
    }
}

fn split(buffer: Vec<u8>) -> &Vec<int> {
    let mut vec = Vec::new();
    let index = 0;
    for x in buffer.iter() {
        if (x == '<') {
            vec.push(index)
        }
        index++;
    }
    return &vec;
}

fn main() {
    let result = io::stdin().read_to_end();
    match result {
        Ok(buffer) => println!("{}", buffer), // vec<u8>
        Err(_) => println!("error")
    }
}