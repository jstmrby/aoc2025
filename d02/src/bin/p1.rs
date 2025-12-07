use std::fs::read_to_string;

fn main() {
    let result: u64 = read_to_string("input.txt")
        .expect("Unable to read file!")
        .trim()
        .split(',')
        .map(|range| {
            let (start, end) = range.split_once('-').expect("Unable to split range");
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();

            (start..=end)
                .filter(|&num| has_matching_halves(num))
                .sum::<u64>()
        })
        .sum();

    println!("{result}")
}

fn has_matching_halves(num: u64) -> bool {
    let num_str = num.to_string();
    if !num_str.len().is_multiple_of(2) {
        return false;
    }

    let midpoint = num_str.len() / 2;
    let (left, right) = num_str.split_at(midpoint);
    left == right
}
