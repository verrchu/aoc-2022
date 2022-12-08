use std::collections::HashSet;

static INPUT: &str = include_str!("input");
const SIDE: usize = 99;

fn main() {
    let mut grid = Vec::<Vec<char>>::new();

    for line in INPUT.lines() {
        grid.push(line.chars().collect());
    }

    let mut seen = HashSet::<(usize, usize)>::new();

    for i in 0..SIDE {
        let mut tallest = grid[i][0];
        seen.insert((i, 0));

        for j in 1..SIDE {
            if grid[i][j] > tallest {
                tallest = grid[i][j];
                seen.insert((i, j));
            }
        }

        let mut tallest = grid[i][SIDE - 1];
        seen.insert((i, SIDE - 1));

        for j in (0..(SIDE - 1)).rev() {
            if grid[i][j] > tallest {
                tallest = grid[i][j];
                seen.insert((i, j));
            }
        }
    }

    for i in 0..SIDE {
        let mut tallest = grid[0][i];
        seen.insert((0, i));

        for j in 1..SIDE {
            if grid[j][i] > tallest {
                tallest = grid[j][i];
                seen.insert((j, i));
            }
        }

        let mut tallest = grid[SIDE - 1][i];
        seen.insert((SIDE - 1, i));

        for j in (0..(SIDE - 1)).rev() {
            if grid[j][i] > tallest {
                tallest = grid[j][i];
                seen.insert((j, i));
            }
        }
    }

    println!("{}", seen.len());
}
