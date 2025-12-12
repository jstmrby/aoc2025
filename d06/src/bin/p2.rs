use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Unable to open file!");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut operations = vec![];
    let mut number_groups: Vec<Vec<u64>> = vec![];
    let mut current_group: Vec<u64> = vec![];
    let mut skip_next = false;

    for col in (0..width).rev() {
        if skip_next {
            skip_next = false;
            continue;
        }

        let column_chars: String = (0..height).map(|row| grid[row][col]).collect();

        let find_operation = column_chars.chars().find(|&c| c == '+' || c == '*');
        if let Some(op) = find_operation {
            operations.push(op);

            let number = parse_number(&column_chars, 1);
            current_group.push(number);

            number_groups.push(current_group);
            current_group = vec![];

            skip_next = true;
        } else {
            let number = parse_number(&column_chars, 0);
            current_group.push(number);
        }
    }

    let result: u64 = number_groups
        .iter()
        .enumerate()
        .map(|(i, numbers)| calculate(operations[i], numbers))
        .sum();

    println!("{result}");
}

fn parse_number(column_chars: &str, offset: usize) -> u64 {
    column_chars[0..column_chars.len() - offset]
        .trim()
        .parse::<u64>()
        .unwrap()
}

fn calculate(operation: char, numbers: &[u64]) -> u64 {
    match operation {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        op => panic!("Unknown operation: {op}"),
    }
}
