use std::fs;

fn main() {
    let input = fs::read_to_string("./bin/01/input").unwrap();

    let max = input
        .trim()
        .split("\n\n")
        .map(|part| {
            part.trim()
                .split("\n")
                .map(|entry| entry.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap();

    println!("{max}");
}
