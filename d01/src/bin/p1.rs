use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Unable to read file!");
    let instructions: Vec<i32> = file
        .trim()
        .lines()
        .map(|line| {
            let (direction, magnitude) = line.split_at(1);
            let value: i32 = magnitude.parse().expect("Unable to parse magnitude");
            if direction == "L" { -value } else { value }
        })
        .collect();

    let mut pointer = 50;
    let result = instructions
        .iter()
        .filter(|&&val| {
            pointer = (pointer + val).rem_euclid(100);
            pointer == 0
        })
        .count();

    println!("{result}")
}
