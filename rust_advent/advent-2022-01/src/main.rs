use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read input file.");
    println!("{contents}");
}
