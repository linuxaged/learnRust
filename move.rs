#![feature(drain)]
fn main() {
    let mut array: Vec<&str> = vec!["mage ", "is ", "a", "hearthstone", "hero"];
    for s in &array {
        println!("{}", s);
    }

    array.drain(1..2);

    for s in &array {
        println!("{}", s);
    }
}