use std::{fs::read_to_string, ops::RangeInclusive};

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read file!");
    let (ranges_str, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<_> = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    ranges.sort_by_key(|r| *r.start());

    let merged_ranges: Vec<RangeInclusive<u64>> =
        ranges.into_iter().fold(Vec::new(), |mut acc, range| {
            if let Some(last) = acc.last_mut()
                && last.end() >= range.start()
            {
                *last = *last.start()..=*range.end().max(last.end());
            } else {
                acc.push(range);
            }

            acc
        });

    let result: u64 = merged_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("{result}");
}
