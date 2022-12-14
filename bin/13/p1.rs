use std::cmp::Ordering;

use itertools::Itertools;
use serde::Deserialize;

static INPUT: &str = include_str!("input");

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum Chunk {
    Many(Vec<Self>),
    One(usize),
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

fn main() {
    let result = INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut packet| {
            let l = serde_json::from_str::<Chunk>(packet.next().unwrap()).unwrap();
            let r = serde_json::from_str::<Chunk>(packet.next().unwrap()).unwrap();

            (l, r)
        })
        .zip(1..)
        .filter_map(|((l, r), i)| (r > l).then_some(i))
        .sum::<usize>();

    println!("{result}");
}
