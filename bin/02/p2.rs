use std::{convert::Infallible, fs, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Action {
    Win,
    Lose,
    Draw,
}

impl FromStr for Action {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        })
    }
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

    fn pick_move(&self, action: Action) -> Self {
        use {Action::*, RPS::*};

        match (self, action) {
            (Rock, Win) => Paper,
            (Rock, Lose) => Scissors,
            (Paper, Win) => Scissors,
            (Paper, Lose) => Rock,
            (Scissors, Win) => Rock,
            (Scissors, Lose) => Paper,
            (m, Draw) => *m,
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

            let l = RPS::from_str(&l).unwrap();
            let r = Action::from_str(&r).unwrap();

            let r = l.pick_move(r);
            r.score() + RPS::cmp(l, r).score()
        })
        .sum::<usize>();

    println!("{score}");
}
