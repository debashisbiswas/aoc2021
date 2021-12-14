use std::fs;

fn main() {
    // Each number has 12 bits.
    let mut counter: [i32; 12] = [0; 12];
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    for line in contents.lines() {
        for (pos, bit) in line.chars().enumerate() {
            let bit = bit.to_digit(2).unwrap();
            counter[pos] += match bit {
                0 => -1,
                1 => 1,
                _ => 0,
            }
        }
    }
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for count in counter {
        // case where both are equally common (count is 0) is unhandled
        if count > 0 {
            gamma_rate += "1";
            epsilon_rate += "0";
        } else {
            gamma_rate += "0";
            epsilon_rate += "1";
        }
    }
    let gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();
    println!("Power consumption: {}", gamma_rate * epsilon_rate);
}
