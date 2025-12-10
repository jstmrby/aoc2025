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
    let mut grid: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut result = 0;

    loop {
        let removes = find_removals(&grid);

        for (r, c) in &removes {
            remove_roll(&mut grid, *r, *c);
        }

        if removes.is_empty() {
            break;
        }

        result += removes.len()
    }

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

fn find_removals(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut removals = Vec::new();
    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '@' && count_adjacent_rolls(grid, row_idx, col_idx) < 4 {
                removals.push((row_idx, col_idx));
            }
        }
    }
    removals
}

fn get_cell(grid: &[Vec<char>], row: usize, col: usize) -> Option<char> {
    grid.get(row)?.get(col).copied()
}

fn remove_roll(grid: &mut [Vec<char>], row: usize, col: usize) {
    grid[row][col] = '.'
}
