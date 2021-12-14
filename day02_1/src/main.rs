use std::fs;

fn main() {
    // https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let mut horizontal = 0;
    let mut depth = 0;
    for line in contents.lines() {
        let (dir, num) = line.split_once(' ').unwrap();
        let num: i32 = num.parse().unwrap();
        match dir {
            "forward" => horizontal += num,
            "down" => depth += num,
            "up" => depth -= num,
            _ => println!("Encountered unexpected input: {}", dir),
        }
    }
    println!("horizontal * depth = {}", horizontal * depth);
}
