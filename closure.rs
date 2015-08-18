#![feature(core, unboxed_closures)]

struct Struct<F> {
    f: F,
}

// `F` is the type of the unboxed closure
// Is an unboxed closure because `F` implements the `FnOnce` trait
impl<F> Struct<F> where F: FnOnce() {
    fn new(f: F) -> Struct<F> {
        Struct {
            f: f,
        }
    }

    fn call(self) {
        self.f.call_once(());
    }
}

fn main() {
    Struct::new(|| println!("Hello")).call();
}