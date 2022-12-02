use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    // Some(true) if b wins
    // Some(false) if b loses
    // None if draw
    fn cmp(a: Self, b: Self) -> Option<bool> {
        match (a, b) {
            (Self::Rock, Self::Paper) => Some(true),
            (Self::Rock, Self::Scissors) => Some(false),
            (Self::Paper, Self::Rock) => Some(false),
            (Self::Paper, Self::Scissors) => Some(true),
            (Self::Scissors, Self::Rock) => Some(true),
            (Self::Scissors, Self::Paper) => Some(false),
            _ => None,
        }
    }

    fn from_left(l: &str) -> Self {
        match l {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        }
    }

    fn from_right(r: &str) -> Self {
        match r {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

trait Score {
    fn score(&self) -> usize;
}

impl Score for RPS {
    fn score(&self) -> usize {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Score for Option<bool> {
    fn score(&self) -> usize {
        match self {
            Some(true) => 6,
            Some(false) => 0,
            None => 3,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./bin/02/input").unwrap();

    let score = input
        .lines()
        .map(|line| {
            let l: String;
            let r: String;

            text_io::scan!(line.bytes() => "{} {}", l, r);

            let l = RPS::from_left(&l);
            let r = RPS::from_right(&r);

            r.score() + RPS::cmp(l, r).score()
        })
        .sum::<usize>();

    println!("{score}");
}
