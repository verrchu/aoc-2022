use std::collections::HashSet;

use itertools::Itertools;

static INPUT: &str = include_str!("input");
const MSG_SIZE: usize = 14;

fn main() {
    let (mut result, _) = INPUT
        .as_bytes()
        .windows(MSG_SIZE)
        .find_position(|seq| HashSet::<&u8>::from_iter(*seq).len() == MSG_SIZE)
        .unwrap();

    result += MSG_SIZE;

    println!("{result}");
}
