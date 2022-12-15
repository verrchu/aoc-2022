use std::collections::HashSet;

static INPUT: &str = include_str!("input");
const TARGET_Y: isize = 2_000_000;

fn main() {
    let input = INPUT.lines()
        .map(|line| {
            let (sx, sy, bx, by): (isize, isize, isize, isize);
            text_io::scan!(line.bytes() => "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by);

            ((sx, sy), (bx, by))
        })
        .collect::<Vec<_>>();

    let mut occupied = HashSet::<isize>::new();
    for ((sx, sy), (bx, by)) in input.iter() {
        let y = (sy - by).abs() + (sx - bx).abs();
        let ydiff = (sy - TARGET_Y).abs();

        if ydiff <= y {
            let level = y - ydiff;
            for x in (sx - level)..=(sx + level) {
                occupied.insert(x);
            }
        }
    }

    for (_s, (bx, by)) in input.iter() {
        if *by == TARGET_Y {
            occupied.remove(bx);
        }
    }

    println!("{}", occupied.len());
}


