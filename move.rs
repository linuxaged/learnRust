#![feature(drain)]
fn main() {
    let mut array: Vec<&str> = vec!["hunter ", "is ", "a", "hearthstone", "hero:"];
    let words = "let the hunt start!";

    let _words = words.replace("start", "begin");

	array.insert(4, &_words);

    for s in &array {
        println!("{}", s);
    }
}