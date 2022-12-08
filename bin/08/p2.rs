use itertools::Itertools;

static INPUT: &str = include_str!("input");
const SIDE: usize = 99;

fn main() {
    let mut grid = Vec::<Vec<char>>::new();

    for line in INPUT.lines() {
        grid.push(line.chars().collect());
    }

    let result = (0..SIDE)
        .cartesian_product(0..SIDE)
        .map(|(row, col)| {
            check_up(&grid, row, col)
                * check_down(&grid, row, col)
                * check_left(&grid, row, col)
                * check_right(&grid, row, col)
        })
        .max()
        .unwrap();

    println!("{result}");
}

fn check_left(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let value = grid[row][col];

    let mut score = 0;

    if col != 0 {
        for i in (0..col).rev() {
            score += 1;
            if grid[row][i] >= value {
                break;
            }
        }
    }

    score
}

fn check_right(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let value = grid[row][col];

    let mut score = 0;

    if col != SIDE - 1 {
        for i in (col + 1)..SIDE {
            score += 1;
            if grid[row][i] >= value {
                break;
            }
        }
    }

    score
}

fn check_up(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let value = grid[row][col];

    let mut score = 0;

    if row != 0 {
        for i in (0..row).rev() {
            score += 1;
            if grid[i][col] >= value {
                break;
            }
        }
    }

    score
}

fn check_down(grid: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let value = grid[row][col];

    let mut score = 0;

    if row != SIDE - 1 {
        for i in (row + 1)..SIDE {
            score += 1;
            if grid[i][col] >= value {
                break;
            }
        }
    }

    score
}
