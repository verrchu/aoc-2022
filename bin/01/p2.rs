use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./bin/01/input").unwrap();

    let max = input
        .trim()
        .split("\n\n")
        .map(|part| {
            part.trim()
                .lines()
                .map(|entry| entry.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .sorted_by(|a, b| b.cmp(&a))
        .take(3)
        .sum::<u64>();

    println!("{max}");
}
