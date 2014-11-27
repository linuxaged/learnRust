use std::io::File;

struct BoyerMoore {
    pat: Vec<u8>,
    source: Vec<u8>,
    delta1: [int, ..256],
    delta2: Vec<u8>,
}

impl BoyerMoore {
    fn new(content: &str, target: &str) -> BoyerMoore {
        BoyerMoore {
            pat: target.to_string().into_bytes(),
            source: content.to_string().into_bytes(),
            delta1: [0, ..256], // actually i want to init with make_delta1()
            delta2: vec![65u8], // init with make_delta2()
        }
    }

    fn make_delta1(&mut self) {
        self.delta1 = [self.pat.len() as int, ..256];

        for i in range(0,self.pat.len()) {
            self.delta1[self.pat[i] as uint] = self.pat.len() as int;
        }
    }

    fn make_delta2(&mut self) {
        // self.delta2 = ...;
    }
}



fn main() {
    let path = Path::new("data.txt");
    let raw_string = File::open(&path).read_to_string().unwrap();
	let mut boyer_moore = BoyerMoore::new(raw_string.as_slice(), "EXAMPLE");
}