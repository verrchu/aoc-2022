use itertools::Itertools;
use std::{convert::Infallible, str::FromStr};

static INPUT: &str = include_str!("input");

fn print_grid(grid: Vec<Vec<char>>) {
    for line in grid {
        println!("{}", line.into_iter().collect::<String>());
    }
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx(isize),
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Op, Self::Err> {
        Ok(match s {
            "noop" => Op::Noop,
            _ => {
                let n = s.strip_prefix("addx ").unwrap().parse().unwrap();

                Self::Addx(n)
            }
        })
    }
}

fn main() {
    let mut grid = vec![vec!['.'; 40]; 6];

    let moves = INPUT
        .lines()
        .map(|line| line.parse::<Op>().unwrap())
        .map(|op| match op {
            Op::Noop => vec![0],
            Op::Addx(n) => vec![0, n],
        })
        .flatten();

    let mut pos = 0;
    for (row, chunk) in moves.chunks(40).into_iter().enumerate() {
        for (i, n) in chunk.enumerate() {
            if (pos..).take(3).contains(&(i as isize)) {
                grid[row][i] = '#';
            }

            pos += n;
        }
    }

    print_grid(grid);
}
