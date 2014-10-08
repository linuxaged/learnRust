use std::io;
use std::vec::Vec;

static key: &'static str = "AaCcGgTtUuMmRrWwSsYyKkVvHhDdBbNn";
static cmp: &'static str = "TTGGCCAAAAKKYYWWSSRRMMBBDDHHVVNN";

fn revcomp(buffer: &mut Vec<u8>, begin: int, end: int) {
    for ii in range(begin, end) {
        let result = key.find(*buffer[ii]);
        match result {
            Some(index) => {
                *buffer[ii] = cmp[index];
            },
            None => ()
        }
    }
}

fn split(buffer: &mut Vec<u8>) {
    let mut split = Vec::new();
    let index = 0;
    let endline = false;
    for x in *buffer.iter() {
        if (x == '<') {
            split.push(index);
            endline = true;
        }
        if (endline) {
            split.push(index);
            endline = false;
        }
        index = index + 1;
    }
    spawn(proc() {
        revcomp(buffer, split[1] + 1, split[2] - 1);
    });
    spawn(proc() {
        revcomp(buffer, split[3] + 1, split[4] - 1);
    });
    spawn(proc() {
        revcomp(buffer, split[5] + 1, split[4] - 1);
    });
}

fn main() {
    let result = io::stdin().read_to_end();
    match result {
        Ok(mut buffer) => split(&buffer), // vec<u8>
        Err(_) => println!("error")
    }
}