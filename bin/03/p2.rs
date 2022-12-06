use itertools::Itertools;
use std::collections::HashSet;

static INPUT: &str = include_str!("input");

trait Priority {
    fn priority(&self) -> usize;
}

impl Priority for char {
    fn priority(&self) -> usize {
        match self {
            'a'..='z' => *self as usize - 96,
            'A'..='Z' => *self as usize - 38,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let result = INPUT
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let common = group
                .map(|item| HashSet::<char>::from_iter(item.chars()))
                .reduce(|acc, x| {
                    HashSet::<char>::from_iter(acc.intersection(&x).map(ToOwned::to_owned))
                })
                .unwrap();

            common.into_iter().next().unwrap().priority()
        })
        .sum::<usize>();

    println!("{result}");
}
