use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to open file!");
    let mut lines = input.lines();
    let operations: Vec<_> = lines.next_back().unwrap().split_whitespace().collect();

    let mut number_grid: Vec<Vec<u64>> = Vec::new();
    for line in lines {
        for (i, nr) in line
            .split_whitespace()
            .map(|nr| nr.parse::<u64>().unwrap())
            .enumerate()
        {
            match number_grid.get_mut(i) {
                Some(numbers) => numbers.push(nr),
                None => number_grid.push(vec![nr]),
            }
        }
    }

    let result: u64 = number_grid
        .iter()
        .enumerate()
        .map(|(i, numbers)| calculate(&operations, numbers, i))
        .sum();

    println!("{result:?}");
}

fn calculate(operations: &[&str], numbers: &[u64], i: usize) -> u64 {
    match operations[i] {
        "+" => numbers.iter().sum(),
        "*" => numbers.iter().product(),
        op => panic!("Unknown operation {op}"),
    }
}
