use std::{cmp::Ordering, str::FromStr};

use serde::Deserialize;

static INPUT: &str = include_str!("input");

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum Chunk {
    Many(Vec<Self>),
    One(usize),
}

impl FromStr for Chunk {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl PartialOrd for Chunk {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Chunk::One(x), Chunk::One(y)) => x.partial_cmp(y),
            (Chunk::Many(x), Chunk::One(y)) => {
                Chunk::Many(x.to_owned()).partial_cmp(&Chunk::Many(vec![Chunk::One(*y)]))
            }
            (Chunk::One(x), Chunk::Many(y)) => {
                Chunk::Many(vec![Chunk::One(*x)]).partial_cmp(&Chunk::Many(y.to_owned()))
            }
            (Chunk::Many(x), Chunk::Many(y)) => {
                for (x, y) in x.iter().zip(y.iter()) {
                    let cmp = x.partial_cmp(y);

                    if cmp != Some(Ordering::Equal) {
                        return cmp;
                    }
                }

                x.len().partial_cmp(&y.len())
            }
        }
    }
}

impl Ord for Chunk {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let mut chunks = INPUT
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Chunk>>();

    let dp1 = "[[2]]".parse::<Chunk>().unwrap();
    let dp2 = "[[6]]".parse::<Chunk>().unwrap();

    chunks.push(dp1.clone());
    chunks.push(dp2.clone());

    chunks.sort();

    let dpi1 = chunks
        .iter()
        .zip(1..)
        .find_map(|(chunk, i)| (chunk == &dp1).then_some(i))
        .unwrap();
    let dpi2 = chunks
        .iter()
        .zip(1..)
        .find_map(|(chunk, i)| (chunk == &dp2).then_some(i))
        .unwrap();

    println!("{}", dpi1 * dpi2);
}
