use std::{collections::HashSet, convert::Infallible, iter, str::FromStr};

static INPUT: &str = include_str!("input");

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
    let target_cycles = HashSet::<isize>::from_iter([20, 60, 100, 140, 180, 220]);

    let result = iter::once(1)
        .chain(
            INPUT
                .lines()
                .map(|line| line.parse::<Op>().unwrap())
                .map(|op| match op {
                    Op::Noop => vec![0],
                    Op::Addx(n) => vec![0, n],
                })
                .flatten(),
        )
        .zip(1..)
        .fold((0, 0), |(mut total_sum, mut target_sum), (n, i)| {
            total_sum += n;
            if target_cycles.contains(&i) {
                target_sum += i * total_sum;
            }

            (total_sum, target_sum)
        })
        .1;

    println!("{result}");
}
