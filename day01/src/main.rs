use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn part1(depths: &Vec<i32>) -> i32 {
    let mut increase_counter = 0;
    let mut last_depth = i32::MAX;
    for (pos, &depth) in depths.iter().enumerate() {
        if depth > last_depth && pos != 0 {
            increase_counter += 1;
        }
        last_depth = depth;
    }
    return increase_counter;
}

fn part2(depths: &Vec<i32>) -> i32 {
    let mut increase_counter = 0;
    let mut last_window = i32::MAX;
    for (pos, &depth) in depths.iter().enumerate() {
        if pos < 2 {
            continue;
        }
        let current_window = depths[pos - 2] + depths[pos - 1] + depth;
        if current_window > last_window {
            increase_counter += 1;
        }
        last_window = current_window;
    }
    return increase_counter;
}

fn main() {
    // https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/open.html
    let path = Path::new("input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        _ => (),
    }

    let depths: Vec<i32> = s.lines().map(|s| s.parse().unwrap()).collect();

    println!("Part 1: Increased {} times.", part1(&depths));
    println!(
        "Part 2: {} sums are larger than the previous sum.",
        part2(&depths)
    );
}
