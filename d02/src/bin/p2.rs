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
                .filter(|&num| has_matching_pattern(num))
                .sum::<u64>()
        })
        .sum();

    println!("{result}")
}

fn has_matching_pattern(num: u64) -> bool {
    let num_str = num.to_string();
    (2..=num_str.len()).any(|batch_count| {
        if !num_str.len().is_multiple_of(batch_count) {
            return false;
        }

        let batch_size = num_str.len() / batch_count;
        (0..batch_count - 1).all(|nth_batch| {
            let current_batch_index = nth_batch * batch_size;
            let lookahead_batch_index = current_batch_index + batch_size;

            let left = &num_str[current_batch_index..lookahead_batch_index];
            let right = &num_str[lookahead_batch_index..lookahead_batch_index + batch_size];

            left == right
        })
    })
}
