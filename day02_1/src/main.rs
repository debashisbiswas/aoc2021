use std::fs;

fn main() {
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    for line in contents.lines() {
        let (dir, num) = line.split_once(' ').unwrap();
    }
}
