use itertools::Itertools;

static STACKS: &str = include_str!("input/stacks");
static MOVES: &str = include_str!("input/moves");

const NSTACKS: usize = 9;
const STACK_STEP: usize = 4;

fn parse_stacks() -> Vec<Vec<char>> {
    let mut stacks = vec![vec![]; NSTACKS];
    STACKS.lines().rev().skip(1).for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(STACK_STEP)
            .enumerate()
            .for_each(|(i, v)| {
                (v != ' ').then(|| stacks[i].push(v));
            })
    });

    stacks
}

fn parse_moves() -> Vec<Move> {
    MOVES
        .lines()
        .map(|line| {
            let (n, from, to): (usize, usize, usize);
            text_io::scan!(line.bytes() => "move {} from {} to {}", n, from, to);

            Move {
                n,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

fn main() {
    let mut stacks = parse_stacks();
    let moves = parse_moves();

    for Move { n, from, to } in moves {
        (0..n).for_each(|_| {
            let v = stacks[from].pop().unwrap();
            stacks[to].push(v);
        })
    }

    let result = stacks
        .iter()
        .flat_map(|stack| stack.last().copied())
        .collect::<String>();
    
    println!("{result}");
}
