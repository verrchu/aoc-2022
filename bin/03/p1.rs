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
        .map(|line| {
            let hlen = line.len() / 2;
            let (l, r) = (&line[0..hlen], &line[hlen..]);

            let l = HashSet::<char>::from_iter(l.chars());
            let r = HashSet::<char>::from_iter(r.chars());

            let mut common = l.intersection(&r);

            common.next().unwrap().priority()
        })
        .sum::<usize>();

    println!("{result}");
}
