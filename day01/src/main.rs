use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    let mut increase_counter = 0;
    let mut last_depth = 0;
    for (pos, &depth) in depths.iter().enumerate() {
        if depth > last_depth && pos != 0 {
            increase_counter += 1;
        }
        last_depth = depth;
    }

    println!("Increased {} times.", increase_counter);
}
