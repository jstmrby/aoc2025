use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("input.txt").expect("Unable to open file!");
    let mut lines = input.trim().lines();

    let starting_position = lines
        .next()
        .unwrap()
        .chars()
        .position(|ch| ch == 'S')
        .unwrap();
    let mut beams = HashSet::from([starting_position]);
    let result = lines.fold(0, |mut split_count, line| {
        let chars: Vec<char> = line.chars().collect();
        let splits: Vec<usize> = beams
            .iter()
            .filter(|&pos| chars[*pos] == '^')
            .copied()
            .collect();

        for pos in splits {
            split_count += 1;
            beams.remove(&pos);
            beams.insert(pos - 1);
            beams.insert(pos + 1);
        }

        split_count
    });

    println!("{result}");
}

