use std::io;

fn main() {
    let grid = io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    println!("{}", visibles(&grid));
    println!("{}", max_scenic_score(&grid));
}

fn visibles(grid: &Vec<Vec<u8>>) -> u32 {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visible(grid, i, j) {
                count += 1;
            }
        }
    }

    count
}

fn max_scenic_score(grid: &Vec<Vec<u8>>) -> u32 {
    let mut max = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            max = std::cmp::max(max, scenic_score(grid, i, j));
        }
    }

    max
}

fn scenic_score(grid: &Vec<Vec<u8>>, row: usize, column: usize) -> u32 {
    let tree = grid[row][column];
    // Edge trees are always visible.
    if row == 0 || column == 0 || row == grid.len() - 1 || column == grid[row].len() - 1 {
        return 0;
    }

    // Check visibility from the left.
    let mut left = 0;
    for i in (0..column).rev() {
        left += 1;
        if grid[row][i] >= tree { break; }
    }

    // Check visibility from the right.
    let mut right = 0;
    for i in column + 1..grid[row].len() {
        right += 1;
        if grid[row][i] >= tree { break; }
    }

    // Check visibility from the top.
    let mut top = 0;
    for i in (0..row).rev() {
        top += 1;
        if grid[i][column] >= tree { break; }
    }

    // Check visibility from the bottom.
    let mut bottom = 0;
    for i in row + 1..grid.len() {
        bottom += 1;
        if grid[i][column] >= tree { break; }
    }

    left * right * top * bottom
}

fn visible(grid: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    let tree = grid[row][column];
    // Edge trees are always visible.
    if row == 0 || column == 0 || row == grid.len() - 1 || column == grid[row].len() - 1 {
        return true;
    }

    // Check visibility from the left.
    let mut left = true;
    for i in 0..column {
        left = grid[row][i] < tree;
        if !left {
            break;
        }
    }

    // Check visibility from the right.
    let mut right = true;
    for i in column + 1..grid[row].len() {
        right = grid[row][i] < tree;
        if !right {
            break;
        }
    }

    // Check visibility from the top.
    let mut top = true;
    for i in 0..row {
        top = grid[i][column] < tree;
        if !top {
            break;
        }
    }

    // Check visibility from the bottom.
    let mut bottom = true;
    for i in row + 1..grid.len() {
        bottom = grid[i][column] < tree;
        if !bottom {
            break;
        }
    }

    left || right || top || bottom
}
