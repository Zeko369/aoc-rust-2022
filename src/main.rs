use std::fs;

const FILENAME: &str = "src/input1.txt";

fn main() {
    let content = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");

    println!("foo: {}", content);
}
