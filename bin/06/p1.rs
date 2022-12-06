use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("input");
const MARKER_SIZE: usize = 4;

fn main() {
    let (mut result, _) = INPUT
        .as_bytes()
        .windows(MARKER_SIZE)
        .find_position(|seq| HashSet::<&u8>::from_iter(*seq).len() == MARKER_SIZE)
        .unwrap();

    result += MARKER_SIZE;

    println!("{result}");
}
