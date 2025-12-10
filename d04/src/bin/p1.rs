use std::fs::read_to_string;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() {
    let input = read_to_string("input.txt").expect("Unable to read file!");
    let grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let result = grid
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter(|&(col_idx, &ch)| {
                    ch == '@' && count_adjacent_rolls(&grid, row_idx, col_idx) < 4
                })
                .count()
        })
        .sum::<usize>();

    println!("{result}");
}

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    DIRECTIONS
        .iter()
        .filter(|&&(dr, dc)| {
            let new_row = row.checked_add_signed(dr);
            let new_col = col.checked_add_signed(dc);

            matches!((new_row, new_col), (Some(r), Some(c)) if get_cell(grid, r, c) == Some('@'))
        })
        .count()
}

fn get_cell(grid: &[Vec<char>], row: usize, col: usize) -> Option<char> {
    grid.get(row)?.get(col).copied()
}
