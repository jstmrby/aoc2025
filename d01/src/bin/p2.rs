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

    let (_, result) = instructions
        .iter()
        .fold((50, 0), |(position, count), &value| {
            let new_position = (position + value).rem_euclid(100);
            let new_count = count + count_zero_crossings(position, value);
            (new_position, new_count)
        });

    println!("{result}")
}

fn count_zero_crossings(start: i32, value: i32) -> i32 {
    let end = (start + value).rem_euclid(100);
    let complete_loops = value.abs() / 100;

    let crosses_zero = if value > 0 {
        ((end < start && start != 0) || end == 0) as i32
    } else {
        ((end > start && start != 0) || end == 0) as i32
    };

    complete_loops + crosses_zero
}
