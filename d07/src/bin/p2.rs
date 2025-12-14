use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Unable to open file!");
    let mut lines = input.trim().lines();

    let starting_position = lines
        .next()
        .unwrap()
        .chars()
        .position(|ch| ch == 'S')
        .unwrap();
    let mut beams = HashMap::from([(starting_position, 1u64)]);

    let result = lines.fold(1u64, |mut overall_timelines, line| {
        let chars: Vec<char> = line.chars().collect();

        let splits: Vec<(usize, u64)> = beams
            .iter()
            .filter(|&(pos, _)| chars[*pos] == '^')
            .map(|(pos, timeline_count)| (*pos, *timeline_count))
            .collect();

        for (pos, count) in splits {
            overall_timelines += count;
            beams.remove(&pos);
            *beams.entry(pos - 1).or_insert(0) += count;
            *beams.entry(pos + 1).or_insert(0) += count;
        }

        overall_timelines
    });

    println!("{result}");
}
