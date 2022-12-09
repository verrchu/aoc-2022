use std::{collections::HashSet, convert::Infallible, str::FromStr};

static INPUT: &str = include_str!("input");
const KNOTS: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Step {
    U(usize),
    D(usize),
    L(usize),
    R(usize),
}

impl Step {
    fn into_parts(self) -> (Direction, usize) {
        match self {
            Self::U(moves) => (Direction::Up, moves),
            Self::D(moves) => (Direction::Down, moves),
            Self::L(moves) => (Direction::Left, moves),
            Self::R(moves) => (Direction::Right, moves),
        }
    }
}

impl FromStr for Step {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction: String;
        let distance: usize;
        text_io::scan!(s.bytes() => "{} {}", direction, distance);

        Ok(match direction.as_str() {
            "U" => Self::U(distance),
            "D" => Self::D(distance),
            "R" => Self::R(distance),
            "L" => Self::L(distance),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    row: isize,
    col: isize,
}

impl Cell {
    fn touches(&self, other: &Cell) -> bool {
        let close_row = ((self.row - 1)..=(self.row + 1))
            .find(|row| *row == other.row)
            .is_some();

        let close_col = ((self.col - 1)..=(self.col + 1))
            .find(|col| *col == other.col)
            .is_some();

        close_row && close_col
    }

    fn step(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.row += 1,
            Direction::Down => self.row -= 1,
            Direction::Left => self.col -= 1,
            Direction::Right => self.col += 1,
        }
    }

    fn step_towards(&mut self, other: &Cell) {
        use std::cmp::Ordering::*;

        match (other.row.cmp(&self.row), other.col.cmp(&self.col)) {
            (Greater, Equal) => self.row += 1,
            (Less, Equal) => self.row -= 1,
            (Equal, Greater) => self.col += 1,
            (Equal, Less) => self.col -= 1,
            (Greater, Greater) => {
                self.row += 1;
                self.col += 1;
            }
            (Greater, Less) => {
                self.row += 1;
                self.col -= 1;
            }
            (Less, Less) => {
                self.row -= 1;
                self.col -= 1;
            }
            (Less, Greater) => {
                self.row -= 1;
                self.col += 1;
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let mut rope = vec![Cell::default(); KNOTS];
    let mut visited = HashSet::<Cell>::new();

    for line in INPUT.lines() {
        let step = line.parse::<Step>().unwrap();
        let (direction, moves) = step.into_parts();

        for _ in 0..moves {
            rope.get_mut(0).unwrap().step(direction);

            for i in 1..KNOTS {
                let prev = rope.get(i - 1).unwrap().clone();
                let next = rope.get_mut(i).unwrap();

                if !next.touches(&prev) {
                    next.step_towards(&prev);
                }
            }

            visited.insert(rope.last().unwrap().to_owned());
        }
    }

    println!("{}", visited.len());
}
