use std::io::File;

struct BoyerMoore {
    pat: []int,
    source: String,
}

impl BoyerMoore {
    fn make_delta1(&self) -> Vec<int> {
        let delta1: [int,..256] = [len, ..256];
        
        for i in range(0,self.pat.len()) {
            delta1[pat[i]] = self.pat.len();
        }
        return delta1;
    }

    fn make_delta2() -> Vec<int> {
        // add code here
    }

    fn boyer_moore(txt: Vec<u8>, target: Vec<u8>) -> bool {
        let len = target.len();

    }
}



fn main() {
    let path = Path::new("data.txt");
    let raw_bytes: Vec<u8> = File::open(&path).read_to_end().unwrap();
	boyer_moore(raw_bytes, "EXAMPLE".to_string().into_bytes());
}