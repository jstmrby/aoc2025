use std::fs::read_to_string;

fn main() {
    let result: u32 = read_to_string("input.txt")
        .expect("Unable to read file!")
        .trim()
        .lines()
        .map(|line| {
            let first_char = line.chars().take(line.len() - 1).max().unwrap();
            let first_pos = line.chars().position(|ch| ch == first_char).unwrap();

            let second_char = line.chars().skip(first_pos + 1).max().unwrap();

            format!("{first_char}{second_char}").parse::<u32>().unwrap()
        })
        .sum();

    println!("{result}")
}

