use std::io::File;

struct BoyerMoore {
    pat: Vec<u8>,
    source: Vec<u8>,
    delta1: [int, ..256],
    delta2: Vec<u8>,
}

impl BoyerMoore {
    fn new(content: &str, target: &str) -> BoyerMoore {
    	let pat: target.to_string().into_bytes();
    	let source: content.to_string().into_bytes();
    	let delta1: BoyerMoore::make_delta1(pat.as_slice());
    	let delta2: BoyerMoore::make_delta2();
        BoyerMoore {
            pat: pat,
            source: source,
            delta1: delta1,
            delta2: delta2,
        }
    }

    fn make_delta1(pat: &[u8]) -> [int, ..256] {
        let mut delta1 = [pat.len() as int, ..256];

        for i in range(0,pat.len()) {
            delta1[pat[i] as uint] = pat.len() as int;
        }

        delta1
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