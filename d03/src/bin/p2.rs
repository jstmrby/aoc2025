use std::fs::read_to_string;

fn main() {
    let result: u64 = read_to_string("input.txt")
        .expect("Unable to read file!")
        .trim()
        .lines()
        .map(|line| {
            let (_, number_str) = (0..12).rev().fold(
                (0, String::with_capacity(12)),
                |(skip, mut number_str), i| {
                    let take = line.len() - skip - i;
                    let chars_in_range: Vec<_> = line.chars().skip(skip).take(take).collect();

                    let max_char = *chars_in_range.iter().max().unwrap();
                    let max_pos = chars_in_range
                        .iter()
                        .position(|&ch| ch == max_char)
                        .unwrap();

                    number_str.push(max_char);
                    (max_pos + skip + 1, number_str)
                },
            );

            number_str.parse::<u64>().unwrap()
        })
        .sum();

    println!("{result}")
}
