use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read file!");
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges_str
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            let start: u64 = start.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            start..=end
        })
        .collect();

    let ids: Vec<u64> = ids_str.lines().map(|line| line.parse().unwrap()).collect();

    let result = ids
        .iter()
        .filter(|&id| ranges.iter().any(|range| range.contains(id)))
        .count();

    println!("{result}");
}
